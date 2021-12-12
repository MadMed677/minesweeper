import * as PIXI from 'pixi.js'
import {WasmCell, CellStatus, MineSweeperEngine,} from '@minesweeper/engine'

import {CellVisual, ICellVisualProps} from './visuals/cell.visual'
import {IVisual} from './visuals/visual.interface'

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

	/**
	 * Contains a map of cell
	 *  - key - cell.id
	 *  - value - instance of IVisual
	 */
	private readonly mapState = new Map<number, IVisual<ICellVisualProps>>()

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
			const entityId = Number(e.target?.name)

			if (!entityId) {
				return
			}

			const cells: Array<WasmCell> = this.minesweeperEngine.uncover(entityId)
			console.log('cells: ', cells)

			const visual = this.mapState.get(entityId)

			if (!visual) {
				throw new Error(`Cannot find visual by id: ${entityId}`)
			}

			visual.setProps({status: CellStatus.Uncovered})
			visual.render()
		})
	}

	private generateField(field: Array<Array<WasmCell>>): void {
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
				const cellVisual = new CellVisual(cell.id)
				cellVisual.setProps({
					position: {
						x: col_index * itemWidth + padding,
						y: row_index * itemHeight + padding,
					},
					size: {
						width: itemWidth - padding,
						height: itemHeight - padding,
					},
					status: cell.status,
					ctype: cell.ctype
				})

				cellVisual.render()

				this.mapState.set(cell.id, cellVisual)
				this.application.stage.addChild(cellVisual.graphics)
			})
		})
	}
}
