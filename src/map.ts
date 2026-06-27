export type Point = {
   x: number,
   y: number
}

type Camera = Point & {
   zoom: number
}

export class GameMap {
   private camera: Camera
   private img: HTMLImageElement
   protected ctx: CanvasRenderingContext2D
   protected scale = 1
   protected x = 0
   protected y = 0
   protected width = 0
   protected height = 0
   protected onclick?: (e: MouseEvent) => void

   constructor(imgSrc: string, private map: HTMLCanvasElement) {
      const ctx = map.getContext("2d")

      if (!ctx) {
         throw new Error("Could not get context 2d from map canvas")
      }

      const img = new Image()
      img.src = imgSrc

      this.img = img
      this.ctx = ctx
      this.camera = {
         x: 0,
         y: 0,
         zoom: 1
      }
   }

   async setup() {
      await this.img.decode()

      let isDragging = false
      const map = this.map

      onresize = () => {
         this.update()
         this.render()
      }

      map.onmousedown = (e) => {
         const cameraPos = this.cameraPosition
         const dragStartX = e.clientX - cameraPos.x
         const dragStartY = e.clientY - cameraPos.y

         onmousemove = (e) => {
            isDragging = true
            this.pan(e.clientX, e.clientY, dragStartX, dragStartY)
         }
      }

      onmouseup = (e) => {
         onmousemove = null

         if (!isDragging) {
            if (this.onclick) {
               this.onclick(e)
            }
         }

         isDragging = false
      }

      map.onwheel = (e) => {
         this.zoom(e.clientX, e.clientY, e.deltaY)
      }

      this.update()
      this.render()
   }

   destroy() {
      this.ctx.reset()
      for (var member in this) delete this[member]
   }

   update() {
      const { map, img } = this

      map.width = map.offsetWidth
      map.height = map.offsetHeight

      const scale = Math.min(map.width / img.width, map.height / img.height)
      const scaledImgWidth = img.width * scale
      const scaledImgHeight = img.height * scale

      const x = (map.width - scaledImgWidth) / 2
      const y = (map.height - scaledImgHeight) / 2

      this.scale = scale
      this.x = x
      this.y = y
      this.width = scaledImgWidth
      this.height = scaledImgHeight
   }

   render() {
      const { img, ctx, camera, x, y, width, height } = this
      
      ctx.reset()

      ctx.translate(camera.x, camera.y)
      ctx.scale(camera.zoom, camera.zoom)

      ctx.drawImage(img, x, y, width, height)
   }

   pan(clientX: number, clientY: number, startX: number, startY: number) {
      this.camera.x = clientX - startX
      this.camera.y = clientY - startY
      this.render()
   }

   zoom(clientX: number, clientY: number, delta: number) {
      const camera = this.camera

      const screenPos = this.clientToScreen(clientX, clientY)
      const worldPos = this.screenToWorld(screenPos.x, screenPos.y)

      const factor = Math.exp(-delta * 0.001)
      const zoom = Math.min(5, Math.max(1, camera.zoom * factor))

      camera.x = screenPos.x - worldPos.x * zoom
      camera.y = screenPos.y - worldPos.y * zoom
      camera.zoom = zoom

      this.render()
   }

   get cameraPosition(): Point {
      return {
         x: this.camera.x,
         y: this.camera.y
      }
   }

   protected clientToScreen(x: number, y: number): Point {
      const rect = this.map.getBoundingClientRect()

      return {
         x: x - rect.left,
         y: y - rect.top
      }
   }

   protected screenToWorld(x: number, y: number): Point {
      const camera = this.camera

      return {
         x: (x - camera.x) / camera.zoom,
         y: (y - camera.y) / camera.zoom
      }
   }

   protected worldToImage(x: number, y: number): Point {
      return {
         x: (x - this.x) / this.scale,
         y: (y - this.y) / this.scale
      }
   }

   protected imageToWorld(x: number, y: number): Point {
      return {
         x: this.x + x * this.scale,
         y: this.y + y * this.scale
      }
   }
}