import * as PIXI from 'pixi.js';
import {
    MineSweeperEngine,
    WasmCell,
    GameStatus,
    GameState,
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

    /** When all textures were loaded set it is as `true` */
    private isTexturesLoaded = false;

    /**
     * Load all textures and executes `callback` when all textures
     *  being loaded
     *
     * @private
     */
    private loadAllTextures(): Promise<void> {
        if (this.isTexturesLoaded) {
            return Promise.resolve();
        }

        return new Promise<void>(resolve => {
            PIXI.Loader.shared
                .add('empty_not_selected', 'assets/minesweeper_00.png')
                .add('empty_selected', 'assets/minesweeper_01.png')
                .add('flagged', 'assets/minesweeper_02.png')
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
                    this.isTexturesLoaded = true;

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

        this.interactionManager = this.application.renderer.plugins
            .interaction as PIXI.InteractionManager;

        this.interactionManager.on('pointerup', this.onCellClick);
    }

    /** Creates a canvas for the battlefield */
    public async createBattlefield(
        rows: number,
        cols: number,
        bombs: number,
    ): Promise<PIXI.Application['view']> {
        /** Updates application by specific column sizes */
        this.application.view.width = COLUMN_SIZE * cols;
        this.application.view.height = COLUMN_SIZE * rows;

        /** Load all textures and generate field with visuals */
        await this.loadAllTextures();

        this.minesweeperEngine = MineSweeperEngine.create(rows, cols, bombs);
        this.minesweeperEngine.onChange(this.onStateChanged);
        this.generateField(this.minesweeperEngine.getField());

        const $flags = document.querySelector('.flags_count');

        const gameState = this.minesweeperEngine.getGameState();

        if ($flags) {
            $flags.textContent = String(gameState.flags);
        }

        return this.application.view;
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
                const props = {
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
                };

                if (cellVisual.shouldComponentUpdate(props)) {
                    cellVisual.setProps(props);
                    cellVisual.render();
                }

                this.mapState.set(cell.id, cellVisual);
                this.application.stage.addChild(cellVisual.graphics);
            });
        });
    }

    /**
     * Fires when user click / touch no the specific cell
     * Accepts Pixi.InteractionEvent
     */
    private onCellClick = (e: PIXI.InteractionEvent): void => {
        /**
         * Assumes that if user uses `altKey` it means that
         *  user wants to flag the field
         */
        const isFlaggedEvent = e.data.originalEvent.altKey;
        const entityId = Number(e.target?.name);

        if (typeof entityId !== 'number' || isNaN(entityId)) {
            return;
        }

        if (isFlaggedEvent) {
            const cell: Readonly<WasmCell> =
                this.minesweeperEngine.flag(entityId);

            const visual = this.mapState.get(cell.id);
            if (!visual) {
                throw new Error(`Cannot find visual by id: ${cell.id}`);
            }

            visual.setProps({
                status: cell.status,
            });
            visual.render();
        } else {
            const cells: ReadonlyArray<Readonly<WasmCell>> =
                this.minesweeperEngine.reveal(entityId);

            cells.forEach(cell => {
                const visual = this.mapState.get(cell.id);

                if (!visual) {
                    throw new Error(`Cannot find visual by id: ${cell.id}`);
                }

                visual.setProps({status: cell.status});
                visual.render();
            });
        }
    };

    /**
     * Fires when the game state has been changed
     *  and we need to update:
     *  - game status
     *  - count of flags on the board
     */
    private onStateChanged = (gameState: GameState): void => {
        if (gameState.status === GameStatus.Lose) {
            alert('Game is over');
            console.warn('Game is over');
        } else if (gameState.status === GameStatus.Won) {
            alert('Game won');
            console.info('Game won');
        }

        const $flags = document.querySelector('.flags_count');
        if ($flags) {
            $flags.textContent = String(gameState.flags);
        }
    };
}
