import * as PIXI from 'pixi.js';
import {
    MineSweeperEngine,
    WasmCell,
    WasmCellState,
    WasmCTypeName,
} from '@minesweeper/engine';

import {CellVisual, ICellVisualProps} from './visuals/cell.visual';
import {IVisual} from './visuals/visual.interface';

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
    });

    /**
     * Load all textures and executes `callback` when all textures
     *  being loaded
     *
     * @private
     */
    private static loadAllTextures(callback: () => void): void {
        PIXI.Loader.shared
            .add('empty_not_selected', 'assets/minesweeper_00.png')
            .add('empty_selected', 'assets/minesweeper_01.png')
            .add('bomb', 'assets/minesweeper_05.png')
            .add('bomb_exploded', 'assets/minesweeper_06.png')
            .add('mark_1', 'assets/minesweeper_08.png')
            .add('mark_2', 'assets/minesweeper_09.png')
            .add('mark_3', 'assets/minesweeper_10.png')
            .add('mark_4', 'assets/minesweeper_11.png')
            .add('mark_5', 'assets/minesweeper_12.png')
            .add('mark_6', 'assets/minesweeper_13.png')
            .add('mark_7', 'assets/minesweeper_14.png')
            .add('mark_8', 'assets/minesweeper_15.png')
            .load(callback);
    }

    private readonly minesweeperEngine: MineSweeperEngine;

    /**
     * Contains a map of cell
     *  - key - cell.id
     *  - value - instance of IVisual
     */
    private readonly mapState = new Map<number, IVisual<ICellVisualProps>>();

    constructor() {
        /** Turn on Pixi inspector */
        window.__PIXI_INSPECTOR_GLOBAL_HOOK__ &&
            window.__PIXI_INSPECTOR_GLOBAL_HOOK__.register({
                PIXI: PIXI,
            });

        document.body.appendChild(this.application.view);

        this.minesweeperEngine = MineSweeperEngine.create(12, 9, 10);

        /** Load all textures and generate field with visuals */
        Application.loadAllTextures(() => {
            this.generateField(this.minesweeperEngine.getField());
        });

        const interactionManager = this.application.renderer.plugins
            .interaction as PIXI.InteractionManager;

        interactionManager.on('pointerup', (e: PIXI.InteractionEvent) => {
            const entityId = Number(e.target?.name);

            if (typeof entityId !== 'number') {
                return;
            }

            const cells: Array<WasmCell> =
                this.minesweeperEngine.reveal(entityId);
            // console.log('cells: ', cells);
            const convertedCells = cells.map(cell => {
                return {
                    status:
                        cell.status === WasmCellState.Revealed
                            ? 'revealed'
                            : 'hidden',
                    name:
                        cell.ctype.name === WasmCTypeName.Mine
                            ? 'mine'
                            : 'cell',
                    value: cell.ctype.value,
                };
            });
            console.log('convertedCells: ', convertedCells);

            cells.forEach(cell => {
                const visual = this.mapState.get(cell.id);

                if (!visual) {
                    throw new Error(`Cannot find visual by id: ${cell.id}`);
                }

                visual.setProps({status: WasmCellState.Revealed});
                visual.render();
            });
        });
    }

    /**
     * Generate and render the battle field
     *  - First array - number of `cols` (x axis)
     *  - Second array - number of `rows` (y axis)
     */
    private generateField(field: Array<Array<WasmCell>>): void {
        /** Cols number is the same as `field` length */
        const gcols = field.length;

        /**
         * Cols number is the same in all rows.
         * Because of that we may take first row and
         *  calculate cols length inside of it
         */
        const grows = field[0].length;
        const padding = 5;
        const itemWidth = (CANVAS_WIDTH - padding) / gcols;
        const itemHeight = (CANVAS_HEIGHT - padding) / grows;

        field.forEach((rows, col_index) => {
            rows.forEach((cell, row_index) => {
                const cellVisual = new CellVisual(cell.id);
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
                    ctype: cell.ctype,
                });

                cellVisual.render();

                this.mapState.set(cell.id, cellVisual);
                this.application.stage.addChild(cellVisual.graphics);
            });
        });
    }
}
