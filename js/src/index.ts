import {MinesweeperClientApplication} from './MinesweeperClientApplication';
import {DifficultyConfig} from './difficulty_config';

const $gameCanvas = document.querySelector<HTMLDivElement>('#game-canvas');
const $gameMenu = document.querySelector<HTMLDivElement>('#game-menu');

enum ButtonElementType {
    Easy = 'game-level_easy',
    Medium = 'game-level_medium',
    Hard = 'game-level_hard',
    Reset = 'game-reset_game',
}

interface IDifficultyConfig {
    rows: number;
    cols: number;
    bombs: number;
}

/** The main entry point into the application */
export class Application {
    private minesweeperApplication!: MinesweeperClientApplication;
    private gameConfig: IDifficultyConfig | undefined;

    /** The first method (aka `main`) to run the code */
    public async run(): Promise<void> {
        this.minesweeperApplication = new MinesweeperClientApplication();

        this.renderTheMenu();
    }

    /** Render the game when user picked up the level of hardness */
    private async renderTheGame(config: IDifficultyConfig): Promise<void> {
        this.gameConfig = config;

        const applicationView =
            await this.minesweeperApplication.createBattlefield(
                config.rows,
                config.cols,
                config.bombs,
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

            const config = (() => {
                switch (buttonId) {
                    case ButtonElementType.Easy: {
                        return DifficultyConfig.easy;
                    }

                    case ButtonElementType.Medium: {
                        return DifficultyConfig.medium;
                    }

                    case ButtonElementType.Hard: {
                        return DifficultyConfig.hard;
                    }

                    default: {
                        return;
                    }
                }
            })();

            if (buttonId === ButtonElementType.Reset) {
                if (!this.gameConfig) {
                    return;
                }

                this.renderTheGame(this.gameConfig);
            }

            if (config) {
                this.renderTheGame(config);
            }
        });
    }
}
