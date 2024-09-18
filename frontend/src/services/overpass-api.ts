 export class OverpassApiService {
  private baseUrl: string;

  constructor(base: string = "https://overpass-api.de/api") {
    this.baseUrl = base;
  }

  public makeSimpleRequest(): Promise<any> {
    return fetch(
      `${this.baseUrl}/interpreter?data=[out:json];area[name="Berlin"]->.searchArea;node["amenity"="restaurant"](area.searchArea);out body;`
    ).then((r) => r.json());
  }
}
