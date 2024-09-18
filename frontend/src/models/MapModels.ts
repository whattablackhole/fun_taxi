export interface Way {
    type: "way";
    id: number;
    nodes: number[];
    tags: {
      highway: string;
      int_name: string;
      maxspeed: number;
      name: string;
      surface: string;
    };
  }
  
  export interface MapNode {
    type: "node";
    id: number;
    lat: number;
    lon: number;
  }
  
  export type MapItem = MapNode | Way;