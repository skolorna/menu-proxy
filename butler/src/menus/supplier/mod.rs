mod kleins;
mod mpi;
mod sabis;
mod skolmaten;
mod sodexo;

use std::str::FromStr;

use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer, Serialize};
use strum::{EnumIter, EnumString};

use super::{day::Day, Menu};

use crate::errors::{ButlerError, ButlerResult};

/// A provider of menus.
#[derive(PartialEq, Debug, Clone, Copy, EnumString, strum::Display, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Supplier {
    Skolmaten,
    Sodexo,
    MPI,
    Kleins,
    Sabis,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupplierInfo {
    pub id: String,
    pub name: String,
}

impl Supplier {
    pub fn id(&self) -> String {
        self.to_string()
    }

    pub fn name(&self) -> String {
        match *self {
            Supplier::Skolmaten => "Skolmaten",
            Supplier::Sodexo => "Sodexo",
            Supplier::MPI => "MPI",
            Supplier::Kleins => "Klein's Kitchen",
            Supplier::Sabis => "Sabis",
        }
        .to_owned()
    }

    pub fn info(&self) -> SupplierInfo {
        SupplierInfo {
            name: self.name(),
            id: self.id(),
        }
    }

    pub async fn list_menus(&self) -> ButlerResult<Vec<Menu>> {
        use Supplier::*;

        match *self {
            Skolmaten => skolmaten::list_menus().await,
            Sodexo => sodexo::list_menus().await,
            MPI => mpi::list_menus().await,
            Kleins => kleins::list_menus().await,
            Sabis => sabis::list_menus().await,
        }
    }

    pub async fn query_menu(&self, menu_id: &str) -> ButlerResult<Menu> {
        use Supplier::*;

        match *self {
            Skolmaten => {
                skolmaten::query_menu(menu_id.parse().map_err(|_| ButlerError::InvalidMenuId)?)
                    .await
            }
            Sodexo => sodexo::query_menu(menu_id).await,
            MPI => mpi::query_menu(menu_id).await,
            Kleins => kleins::query_menu(menu_id).await,
            Sabis => sabis::query_menu(menu_id).await,
        }
    }

    pub async fn list_days(
        &self,
        menu_id: &str,
        first: NaiveDate,
        last: NaiveDate,
    ) -> ButlerResult<Vec<Day>> {
        use Supplier::*;

        match *self {
            Skolmaten => {
                skolmaten::list_days(
                    menu_id.parse().map_err(|_| ButlerError::InvalidMenuId)?,
                    first,
                    last,
                )
                .await
            }
            Sodexo => sodexo::list_days(menu_id, first, last).await,
            MPI => mpi::list_days(menu_id, first, last).await,
            Kleins => kleins::list_days(menu_id, first, last).await,
            Sabis => sabis::list_days(menu_id, first, last).await,
        }
    }
}

impl Serialize for Supplier {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Supplier {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_str() {
        let s = Supplier::Skolmaten.to_string();
        assert_eq!(s, "skolmaten");
        assert_eq!(Supplier::from_str(&s).unwrap(), Supplier::Skolmaten);
        assert!(Supplier::from_str("skolmat").is_err());
    }

    #[test]
    fn ser_de() {
        let s = serde_json::to_string(&Supplier::Skolmaten).unwrap();
        assert_eq!(s, "\"skolmaten\"");
        assert_eq!(
            serde_json::from_str::<Supplier>(&s).unwrap(),
            Supplier::Skolmaten
        );
        assert!(serde_json::from_str::<Supplier>("\"bruh\"").is_err());
    }

    #[tokio::test]
    async fn sodexo_query_menu() {
        assert_eq!(
            Supplier::Sodexo
                .query_menu("e8851c61-013b-4617-93d9-adab00820bcd")
                .await
                .unwrap()
                .title,
            "Södermalmsskolan, Södermalmsskolan"
        );
        assert!(Supplier::Sodexo.query_menu("bruh").await.is_err());
    }

    #[tokio::test]
    async fn kleins_query_menu() {
        let menu = Supplier::Kleins
            .query_menu("viktor-rydberg-grundskola-jarlaplan")
            .await
            .unwrap();
        assert_eq!(menu.title, "Viktor Rydberg Gymnasium Jarlaplan");
        assert!(Supplier::Kleins.query_menu("nonexistent").await.is_err());
    }
}