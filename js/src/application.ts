import * as PIXI from 'pixi.js'
import {
	MineSweeperEngine,
	Cell,
} from '@minesweeper/engine'

const CANVAS_WIDTH = 1000;
const CANVAS_HEIGHT = 1000;

declare global {
	interface Window {
		__PIXI_INSPECTOR_GLOBAL_HOOK__: {
			register: (data: {PIXI: unknown}) => void;
		};
	}
}

export class Application {
	private readonly application = new PIXI.Application({
		width: CANVAS_WIDTH,
		height: CANVAS_HEIGHT,
		backgroundColor: 2174781,
	})

	private readonly minesweeperEngine: MineSweeperEngine

	constructor() {
		window.__PIXI_INSPECTOR_GLOBAL_HOOK__ &&
			window.__PIXI_INSPECTOR_GLOBAL_HOOK__.register({
				PIXI: PIXI,
			})

		document.body.appendChild(this.application.view)

		this.minesweeperEngine = MineSweeperEngine.create(12, 9)

		this.generateField(this.minesweeperEngine.getField())

		const interactionManager = this.application.renderer.plugins.interaction as PIXI.InteractionManager

		interactionManager.on('pointerup', (e: PIXI.InteractionEvent) => {
			const entityId = Number(e.target.name)

			console.log('entityId: ', entityId)
			const cell = this.minesweeperEngine.uncover(entityId)
			console.log('cell: ', cell)
		})
	}

	private generateField(field: Array<Array<Cell>>): void {
		/** Rows number is the same as `field` length */
		const grows = field.length

		/**
		 * Cols number is the same in all rows.
		 * Because of that we may take first row and
		 *  calculate cols length inside of it
		 */
		const gcols = field[0].length
		const padding = 5
		const itemWidth = (CANVAS_WIDTH - padding) / gcols
		const itemHeight = (CANVAS_HEIGHT - padding) / grows

		field.forEach((rows, row_index) => {
			rows.forEach((cell, col_index) => {
				const rect = new PIXI.Graphics()
				this.application.stage.addChild(rect)

				rect.beginFill(PIXI.utils.string2hex('#FCF3CF'))
				rect.drawRect(col_index * itemWidth + padding, row_index * itemHeight + padding, itemWidth - padding, itemHeight - padding)
				rect.endFill()

				rect.interactive = true
				rect.buttonMode = true

				rect.name = String(cell.getId())
			})
		})
	}
}
