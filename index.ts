let isPlaying = false

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

   ctx.drawImage(img, x, y, img.width * scale, img.height * scale)
}

function startGame() {
   const img = new Image()

   img.onload = () => {
      drawMap(img)

      window.addEventListener("resize", () => {
         drawMap(img)
      })
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
