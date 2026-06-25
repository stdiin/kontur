import { Game } from "./game.js"

const mapCanvas = document.getElementById("map") as HTMLCanvasElement
const game = document.getElementById("game") as HTMLDivElement
const editor = document.getElementById("editor") as HTMLDivElement
const mainMenu = document.getElementById("main-menu") as HTMLDivElement
const playButton = document.getElementById("play-button") as HTMLButtonElement
const leaveButton = document.getElementById("leave-button") as HTMLButtonElement

function toggleUI(mode: "MENU" | "GAME" | "EDITOR") {
   switch(mode) {
      case "MENU":
         game.classList.add("hidden")
         editor.classList.add("hidden")
         mainMenu.classList.remove("hidden")
         break

      case "GAME":
         mainMenu.classList.add("hidden")
         game.classList.remove("hidden")
         break

      case "EDITOR":
         mainMenu.classList.add("hidden")
         editor.classList.remove("hidden")
         break
   }
}

playButton.onclick = () => {
   toggleUI("GAME")

   const img = new Image()
   const markerImg = new Image()
   markerImg.src = "assets/map-marker.svg"

   img.onload = () => {
      const map = new Game(img, mapCanvas, markerImg)
      let isDragging = false

      onresize = () => {
         map.update()
         map.render()
      }

      onmousedown = (e) => {
         const cameraPos = map.cameraPosition
         const dragStartX = e.clientX - cameraPos.x
         const dragStartY = e.clientY - cameraPos.y

         onmousemove = (e) => {
            isDragging = true
            map.pan(e.clientX, e.clientY, dragStartX, dragStartY)
         }
      }

      onmouseup = (e) => {
         onmousemove = null

         if (!isDragging) {
            map.select(e.clientX, e.clientY)
         }

         isDragging = false
      }

      onwheel = (e) => {
         map.zoom(e.clientX, e.clientY, e.deltaY)
      }

      map.update()
      map.render()
   }

   img.src = "assets/north-america-map.png"
}

leaveButton.onclick = () => {
   toggleUI("MENU")
}
