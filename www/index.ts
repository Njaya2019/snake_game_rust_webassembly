

import init, {World, Direction, GameStatus} from "snake_game";
import { rnd } from "./utils/rnd";

init().then(wasm => {

    const CELL_SIZE = 20;
    const WORLD_WIDTH = 32;
    const SNAKE_SPAWN_IDX = rnd(WORLD_WIDTH * WORLD_WIDTH);
    const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
    const worldWidth = world.width()
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    const points = document.getElementById("points")
    const gameStatus = document.getElementById("game-status")
    const gameControlBtn = document.getElementById("game-control-btn")

    // For demo to illustrate how to read from a pointer
    // const snakeCellPtr = world.snake_cells()
    // const snakeLength = world.snake_length()

    // const snakeCells = new Uint32Array(
    //     wasm.memory.buffer,
    //     snakeCellPtr,
    //     snakeLength
    // )
    // console.log(snakeCells)

    gameControlBtn.addEventListener("click", ()=>{
        // alert("clicked")
        const status = world.game_status()

        if (status === undefined){
            gameControlBtn.textContent = "Playing..."
            world.start_game()
            play()
        }
        else{
            location.reload()
        }
    })

    document.addEventListener("keydown", (event) => {
        switch(event.code){
            case "ArrowUp":
                world.change_snake_direction(Direction.Up)
                // console.log("Change direction to up")
                break;
            case "ArrowRight":
                world.change_snake_direction(Direction.Right)
                // console.log("Change direction to right")
                break;
            case "ArrowDown":
                world.change_snake_direction(Direction.Down)
                // console.log("Change direction to down")
                break;
            case "ArrowLeft":
                world.change_snake_direction(Direction.Left)
                // console.log("Change direction to left")
                break;
        }
    })

    function drawWorld() {
        ctx.beginPath();

        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0)
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE) 
        }

        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y)
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)  
        }

        ctx.stroke();
    }

    function drawReward() {
        const idx = world.reward_cell()
        const col = idx % worldWidth
        const row = Math.floor(idx / worldWidth)

        ctx.beginPath();
        ctx.fillStyle = "#FF0000"

        ctx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        )
        ctx.stroke();
    }

    function drawSanke() {
        // Don't need this anymore because we have the snake cells
        // which we've got through the pointer
        // const snakeIdx = world.snake_head_idx()

        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length()
        )

        snakeCells
            .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
            .forEach((cellIdx, i) => {
            const col = cellIdx % worldWidth;
            const row = Math.floor(cellIdx / worldWidth)

            ctx.fillStyle = i === 0 ? "#7878db" : "#000000"
            ctx.beginPath();
    
            ctx.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            )

        })

        //  Move up immediate to render each cell for the snake
        // const col = snakeIdx % worldWidth;
        // const row = Math.floor(snakeIdx / worldWidth)

        // ctx.beginPath();

        // ctx.fillRect(
        //     col * CELL_SIZE,
        //     row * CELL_SIZE,
        //     CELL_SIZE,
        //     CELL_SIZE
        // )
        ctx.stroke();
    }

    function drawGameStatus() {
        // const status = world.game_status()
        gameStatus.textContent = world.game_status_text()
        points.textContent = world.points().toString()

        // if (status === GameStatus.Won || status === GameStatus.Lost){
        //     gameStatus.textContent = "Re-play"
        // }
    }

    function paint() {
        drawWorld()
        drawSanke()
        drawReward()
        drawGameStatus()
    }

    function play() {
        const status = world.game_status()

        if (status === GameStatus.Won || status === GameStatus.Lost){
            gameStatus.textContent = "Re-play"
        }

        const fps = 3
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height)
            // Changed from update to step to allow moving of the snake
            // world.update()
            world.step()
            paint()
            requestAnimationFrame(play)
        }, 1000 / fps)
        
    }

    paint()
    // update()
})
























// function hello(hello) {
//     console.log(hello);
// }

// hello("Hello World")

// async function init() {
//     // const byteArray = new Int8Array([0x00])
//     const response = await fetch("test.wasm");
//     const buffer = await response.arrayBuffer();
//     const wasm = await WebAssembly.instantiate(buffer);
//     const sumFunction = wasm.instance.exports.sum;
//     const result = sumFunction(19, 23);
//     console.log(result)
// }

// init()