let isPlaying = false
let cameraOffsetX = 0
let cameraOffsetY = 0
let zoom = 1

const MAX_ZOOM = 3
const MIN_ZOOM = 1

const game = document.getElementById("game") as HTMLDivElement
const mainMenu = document.getElementById("main-menu") as HTMLDivElement
const map = document.getElementById("map") as HTMLCanvasElement
const ctx = map.getContext("2d")!

function drawMap(img: HTMLImageElement) {
   map.width = map.offsetWidth
   map.height = map.offsetHeight

   const scale = Math.min(map.width / img.width, map.height / img.height)
   const x = (map.width - img.width * scale) / 2
   const y = (map.height - img.height * scale) / 2

   ctx.translate(cameraOffsetX, cameraOffsetY)
   ctx.scale(zoom, zoom)

   ctx.drawImage(img, x, y, img.width * scale, img.height * scale)
}

function startGame() {
   const img = new Image()

   img.onload = () => {
      drawMap(img)

      onresize = () => {
         drawMap(img)
      }

      onmousedown = (event) => {
         const dragStartX = event.clientX - cameraOffsetX
         const dragStartY = event.clientY - cameraOffsetY
         
         onmousemove = (event) => {
            cameraOffsetX = event.clientX - dragStartX
            cameraOffsetY = event.clientY - dragStartY
            drawMap(img)
         }
      }

      onmouseup = () => {
         onmousemove = null
      }

      onwheel = (event) => {
         const rect = map.getBoundingClientRect()
         
         const mouseX = event.clientX - rect.left
         const mouseY = event.clientY - rect.top

         const worldX = (mouseX - cameraOffsetX) / zoom
         const worldY = (mouseY - cameraOffsetY) / zoom

         zoom *= Math.exp(-event.deltaY * 0.001)
         zoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom))

         cameraOffsetX = mouseX - worldX * zoom
         cameraOffsetY = mouseY - worldY * zoom
         
         drawMap(img)
      }
   }

   img.src = "assets/north-america-map.png"

   game.classList.remove("hidden")
   mainMenu.classList.add("hidden")
   isPlaying = true
}

function endGame() {
   game.classList.add("hidden")
   mainMenu.classList.remove("hidden")
   isPlaying = false
}
