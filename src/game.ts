import { GameMap, Point } from "./map.js";

type Marker = Point & {
   img: HTMLImageElement,
   size: number,
}

export class Game extends GameMap {
   private marker: Marker

   constructor(mapImg: HTMLImageElement, map: HTMLCanvasElement, markerImg: HTMLImageElement) {
      super(mapImg, map)
      this.marker = {
         x: 0,
         y: 0,
         size: 40,
         img: markerImg
      }
   }

   render() {
      super.render()

      const marker = this.marker

      if (marker.x === 0 && marker.y === 0) return

      const x = marker.x - marker.size / 2
      const y = marker.y - marker.size

      this.ctx.drawImage(marker.img, x, y, marker.size, marker.size)
   }

   select(clientX: number, clientY: number) {
      const screenPos = this.clientToScreen(clientX, clientY)
      const worldPos = this.screenToWorld(screenPos.x, screenPos.y)

      if (!this.isPointInsideMap(worldPos.x, worldPos.y)) return

      this.marker.x = worldPos.x
      this.marker.y = worldPos.y

      this.render()
   }

   private isPointInsideMap(x: number, y: number) {
      return x >= this.x &&
         x <= this.x + this.width &&
         y >= this.y &&
         y <= this.y + this.height
   }
}