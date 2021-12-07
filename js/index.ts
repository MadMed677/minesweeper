import * as PIXI from 'pixi.js'
import {
	MineSweeperEngine
} from '@minesweeper/engine'

const application = new PIXI.Application()

document.body.appendChild(application.view)

const rect = new PIXI.Graphics()
application.stage.addChild(rect)

rect.beginFill(PIXI.utils.string2hex('#FCF3CF'))
rect.drawRect(0, 0, 100, 100)
rect.endFill()

console.log('Minesweeper Engine initialized: ', MineSweeperEngine.create(10, 10));
