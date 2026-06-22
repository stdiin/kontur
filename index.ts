let isPlaying = false
let cameraOffsetX = window.innerWidth / 2
let cameraOffsetY = window.innerHeight / 2

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

   ctx.translate(-window.innerWidth / 2 + cameraOffsetX, -window.innerHeight / 2 + cameraOffsetY)
   
   ctx.drawImage(img, x, y, img.width * scale, img.height * scale)
}

function startGame() {
   const img = new Image()

   img.onload = () => {
      drawMap(img)

      onresize = () => {
         drawMap(img)
      }

      let dragStartX = 0
      let dragStartY = 0

      onmousedown = (event) => {
         dragStartX = event.clientX - cameraOffsetX
         dragStartY = event.clientY - cameraOffsetY
         
         onmousemove = (event) => {
            cameraOffsetX = event.clientX - dragStartX
            cameraOffsetY = event.clientY - dragStartY
            drawMap(img)
         }
      }

      onmouseup = () => {
         onmousemove = null
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
