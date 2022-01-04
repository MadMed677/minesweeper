import * as PIXI from 'pixi.js';
import {
    GameState,
    MineSweeperEngine,
    WasmCell,
    WasmCellState,
} from '@minesweeper/engine';

import {CellVisual, ICellVisualProps} from './visuals/cell.visual';
import {IVisual} from './visuals/visual.interface';

const DEFAULT_CANVAS_WIDTH = 1000;
const DEFAULT_CANVAS_HEIGHT = 1000;

/** Size of the column (width or height) in pixels */
const COLUMN_SIZE = 40;

declare global {
    interface Window {
        __PIXI_INSPECTOR_GLOBAL_HOOK__: {
            register: (data: {PIXI: unknown}) => void;
        };
    }
}

export class MinesweeperClientApplication {
    private readonly application = new PIXI.Application({
        width: DEFAULT_CANVAS_WIDTH,
        height: DEFAULT_CANVAS_HEIGHT,
        backgroundColor: 2174781,
    });

    rows!: number;
    cols!: number;
    bombs!: number;

    /**
     * Load all textures and executes `callback` when all textures
     *  being loaded
     *
     * @private
     */
    private static loadAllTextures(): Promise<void> {
        return new Promise<void>(resolve => {
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
                .load(() => {
                    resolve();
                });
        });
    }

    private minesweeperEngine!: MineSweeperEngine;
    private readonly interactionManager: PIXI.InteractionManager;

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

        const $gameCanvas =
            document.querySelector<HTMLCanvasElement>('#game-canvas');
        const $resetButton =
            document.querySelector<HTMLButtonElement>('#game-reset_game');

        if (!$gameCanvas) {
            throw new Error('Cannot find "#game-canvas" element on the page');
        }

        if (!$resetButton) {
            throw new Error(
                'Cannot find "#game-reset_game" element on the page',
            );
        }

        $gameCanvas.appendChild(this.application.view);
        $resetButton.addEventListener('click', () => {
            this.minesweeperEngine = MineSweeperEngine.create(
                this.rows,
                this.cols,
                this.bombs,
            );
            this.generateField(this.minesweeperEngine.getField());
        });

        this.interactionManager = this.application.renderer.plugins
            .interaction as PIXI.InteractionManager;
    }

    /** Creates a canvas for the battlefield */
    public async createBattlefield(rows: number, cols: number, bombs: number) {
        this.rows = rows;
        this.cols = cols;
        this.bombs = bombs;

        /** Updates application by specific column sizes */
        this.application.view.width = COLUMN_SIZE * cols;
        this.application.view.height = COLUMN_SIZE * rows;

        /** Load all textures and generate field with visuals */
        await MinesweeperClientApplication.loadAllTextures();

        this.minesweeperEngine = MineSweeperEngine.create(rows, cols, bombs);
        this.generateField(this.minesweeperEngine.getField());

        this.interactionManager.on('pointerup', (e: PIXI.InteractionEvent) => {
            const entityId = Number(e.target?.name);

            if (typeof entityId !== 'number') {
                return;
            }

            const cells: Array<WasmCell> =
                this.minesweeperEngine.reveal(entityId);

            if (this.minesweeperEngine.getGameState() === GameState.Lose) {
                console.warn('Game is over');
            }

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
        field.forEach((rows, col_index) => {
            rows.forEach((cell, row_index) => {
                const cellVisual = new CellVisual(cell.id);
                cellVisual.setProps({
                    position: {
                        x: col_index * COLUMN_SIZE,
                        y: row_index * COLUMN_SIZE,
                    },
                    size: {
                        width: COLUMN_SIZE,
                        height: COLUMN_SIZE,
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
