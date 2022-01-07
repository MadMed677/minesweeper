import {MinesweeperClientApplication} from './MinesweeperClientApplication';

const $gameCanvas = document.querySelector<HTMLDivElement>('#game-canvas');
const $gameMenu = document.querySelector<HTMLDivElement>('#game-menu');

enum ButtonElementType {
    Easy = 'game-level_easy',
    Medium = 'game-level_medium',
    Hard = 'game-level_hard',
}

/** The main entry point into the application */
export class Application {
    private minesweeperApplication!: MinesweeperClientApplication;

    /** The first method (aka `main`) to run the code */
    public async run(): Promise<void> {
        this.minesweeperApplication = new MinesweeperClientApplication();

        this.renderTheMenu();
    }

    /** Render the game when user picked up the level of hardness */
    private async renderTheGame(
        rows: number,
        cols: number,
        bombs: number,
    ): Promise<void> {
        const applicationView =
            await this.minesweeperApplication.createBattlefield(
                rows,
                cols,
                bombs,
            );

        if (!$gameCanvas) {
            throw new Error('Cannot find "#game-canvas" element on the page');
        }

        $gameCanvas.appendChild(applicationView);
    }

    /** Render the menu to pick up the level of hardness */
    private renderTheMenu(): void {
        if (!$gameMenu) {
            throw new Error('Cannot find "#game-menu" element on the page');
        }

        $gameMenu.addEventListener('click', e => {
            const button = e.target as HTMLButtonElement;
            const buttonId = button.id as ButtonElementType;

            switch (buttonId) {
                case ButtonElementType.Easy: {
                    this.renderTheGame(10, 7, 10);

                    break;
                }

                case ButtonElementType.Medium: {
                    this.renderTheGame(12, 9, 10);

                    break;
                }

                case ButtonElementType.Hard: {
                    this.renderTheGame(15, 10, 20);

                    break;
                }

                default: {
                    console.log('non covered', buttonId);

                    break;
                }
            }
        });
    }
}
