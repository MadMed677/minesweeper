import {
	MineSweeperEngine
} from '@minesweeper/engine'

import {Application} from './application'

new Application()

const mineSweeperEngine = MineSweeperEngine.create(10, 10)
console.log(mineSweeperEngine.getField())
