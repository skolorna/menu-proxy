import { DistrictsResponse, ProvincesResponse, SkolmatenStationsResponse } from "./types";
import performSkolmatenRequest from "./request";
import { ListMenus, ProviderMenu } from "../types";

export function validateSchoolName(name: string): boolean {
	return !/info/i.test(name);
}

export const getSkolmatenSchools: ListMenus = async () => {
	const { provinces } = await performSkolmatenRequest<ProvincesResponse>("/provinces");

	const menus3d: ProviderMenu[][][] = await Promise.all(
		provinces.map(async (province) => {
			const { districts } = await performSkolmatenRequest<DistrictsResponse>(`/districts?province=${province.id}`);

			return Promise.all(
				districts.map(async (district) => {
					const { stations } = await performSkolmatenRequest<SkolmatenStationsResponse>(
						`/stations?district=${district.id}`,
					);

					return stations.map(({ id, name }) => ({
						id: id.toString(),
						title: name,
					}));
				}),
			);
		}),
	);

	const schools = menus3d.flat(2).filter(({ title: name }) => validateSchoolName(name));

	return schools;
};
