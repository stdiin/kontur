let isPlaying = false

const game = document.getElementById("game") as HTMLDivElement
const mainMenu = document.getElementById("main-menu") as HTMLDivElement
const map = document.getElementById("map") as HTMLCanvasElement
const ctx = map.getContext("2d")!

function drawMap(img: HTMLImageElement) {
   img.onload = () => {
      map.width = map.offsetWidth
      map.height = map.offsetHeight
      ctx.imageSmoothingQuality = "high"
      ctx.drawImage(img, window.innerWidth / 2 - img.width / 2, window.innerHeight / 2 - img.height / 2)
   }
}

function startGame() {
   const img = new Image()
   img.src = "assets/north-america-map.png"

   drawMap(img)

   game.classList.remove("hidden")
   mainMenu.classList.add("hidden")
   isPlaying = true
}

function endGame() {
   game.classList.add("hidden")
   mainMenu.classList.remove("hidden")
   isPlaying = false
}
