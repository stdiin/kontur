let isPlaying = false

const game = document.getElementById("game")
const mainMenu = document.getElementById("main-menu")

function startGame() {
   if (!game || !mainMenu) {
      return
   }

   console.log("game started")

   game.classList.remove("hidden")
   mainMenu.classList.add("hidden")

   isPlaying = true
}

function endGame() {
   console.log("game ended")
   game?.classList.add("hidden")
   mainMenu?.classList.remove("hidden")
   isPlaying = false
}
