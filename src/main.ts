import { Game } from "./game.js"

const mapCanvas = document.getElementById("map") as HTMLCanvasElement
const game = document.getElementById("game") as HTMLDivElement
const editor = document.getElementById("editor") as HTMLDivElement
const mainMenu = document.getElementById("main-menu") as HTMLDivElement
const mapSelector = document.getElementById("map-selector") as HTMLDivElement
const mapUpload = document.getElementById("map-upload") as HTMLInputElement
const playButton = document.getElementById("play-button") as HTMLButtonElement
const leaveButton = document.getElementById("leave-button") as HTMLButtonElement

let map: Game | null

function toggleUI(mode: "MENU" | "GAME" | "EDITOR") {
   switch(mode) {
      case "MENU":
         game.classList.add("hidden")
         editor.classList.add("hidden")
         mapSelector.classList.add("hidden")
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

   mapSelector.classList.remove("hidden")
   mapUpload.onchange = () => {
      const mapData = mapUpload.files?.item(0)

      if (!mapData) return

      mapData.text()
         .then(text => JSON.parse(text))
         .then(json => {
            mapSelector.classList.add("hidden")

            const mapImgData = json.image

            if (!mapImgData) {
               throw new Error("Invalid map data")
            }

            map = new Game(mapImgData, mapCanvas, "assets/map-marker.svg")
            map.setup()
         })
   }
}

leaveButton.onclick = () => {
   toggleUI("MENU")
   map?.destroy()
}
