import { MapItem, MapNode } from "../../models/MapModels";
import { Neighbor } from "./path-finder-interfaces";



export class PathFinderService {
  private graph: Map<number, { adjacent: Neighbor[] }> = new Map();
  private nodes: Map<number, MapNode> = new Map();

  public getNodes(): MapNode[] {
    return [...this.nodes.values()];
  }

  public findShortestPath(
    start: { lat: number; lon: number },
    end: { lat: number; lon: number }
  ) {
    const startNode = this.findNearestNode(start.lat, start.lon);
    const endNode = this.findNearestNode(end.lat, end.lon);

    if (!startNode || !endNode) {
      throw new Error("Start or end node not found");
    }

    const distances: Map<number, number> = new Map();
    const previous: Map<number, number | null> = new Map();
    const visited: Set<number> = new Set();
    const pq: Array<{ id: number; distance: number }> = [];

    this.nodes.forEach((node) => {
      distances.set(node.id, Infinity);
      previous.set(node.id, null);
    });

    distances.set(startNode.id, 0);

    pq.push({ id: startNode.id, distance: 0 });

    while (pq.length > 0) {
      pq.sort((a, b) => a.distance - b.distance);
      const current = pq.shift();
      if (!current || visited.has(current.id)) continue;

      visited.add(current.id);

      if (current.id === endNode.id) {
        return this.constructPath(previous, endNode.id);
      }

      const neighbors = this.graph.get(current.id)?.adjacent || [];

      for (const neighbor of neighbors) {
        if (visited.has(neighbor.id)) continue;

        const altDistance = distances.get(current.id)! + (neighbor.distance ?? 0);

        if (altDistance < distances.get(neighbor.id)!) {
          distances.set(neighbor.id, altDistance);
          previous.set(neighbor.id, current.id);
          pq.push({ id: neighbor.id, distance: altDistance });
        }
      }
    }

    return [];
  }


  public buildGraph(mapItems: MapItem[]) {
    mapItems.forEach((mapItem) => {
      if (mapItem.type === "way" && ["service"].includes(mapItem.tags.highway)) {
        return;
      }

      if (mapItem.type == "node") {
        this.nodes.set(mapItem.id, mapItem);
      } else {
        let nodes = mapItem.nodes ?? [];
        for (let i = 0; i < nodes.length; i++) {
          if (!this.graph.has(nodes[i])) {
            this.graph.set(nodes[i], { adjacent: [] });
          }

          if (i + 1 < mapItem.nodes.length) {
            let adjacent = nodes[i + 1];
            this.graph
              .get(nodes[i])
              ?.adjacent.push({ id: adjacent, distance: undefined });
          }

          if (i - 1 >= 0) {
            let adjacent = nodes[i - 1];
            this.graph
              .get(nodes[i])
              ?.adjacent.push({ id: adjacent, distance: undefined });
          }
        }
      }
    });

    this.graph.forEach((n, k) => {
      let current = this.nodes.get(k);
      if (!current) {
        return;
      }
      n.adjacent.forEach((neigbour) => {
        let nNode = this.nodes.get(neigbour.id);
        if (!nNode) {
          return;
        }
        neigbour.distance = this.haversineDistance(
          current.lat,
          current.lon,
          nNode.lat,
          nNode.lon
        );
      });
    });
  }

  private haversineDistance(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number
  ): number {
    const R = 6371;
    const toRadians = (degrees: number) => degrees * (Math.PI / 180);

    const dLat = toRadians(lat2 - lat1);
    const dLon = toRadians(lon2 - lon1);

    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(toRadians(lat1)) *
        Math.cos(toRadians(lat2)) *
        Math.sin(dLon / 2) *
        Math.sin(dLon / 2);

    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    const distance = R * c;

    return distance;
  }

  private findNearestNode(lat: number, lon: number) {
    let min = Infinity;
    let node_id = NaN;

    this.nodes.forEach((n) => {
      let distance = this.haversineDistance(n.lat, n.lon, lat, lon);

      if (min > distance) {
        node_id = n.id;
        min = distance;
      }
    });

    return this.nodes.get(node_id);
  }

  private constructPath(previous: Map<number, number | null>, endNodeId: number) {
    const path: MapNode[] = [];
    let currentNode: number | null = endNodeId;

    while (currentNode !== null) {
      path.unshift(this.nodes.get(currentNode)!);
      currentNode = previous.get(currentNode) || null;
    }

    return path;
  }
}
