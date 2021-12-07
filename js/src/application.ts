import * as PIXI from 'pixi.js'

declare global {
	interface Window {
		__PIXI_INSPECTOR_GLOBAL_HOOK__: {
			register: (data: {PIXI: unknown}) => void;
		};
	}
}

export class Application {
	private readonly application = new PIXI.Application({
		width: 1000,
		height: 1000,
		backgroundColor: 2174781,
	})

	constructor() {
		window.__PIXI_INSPECTOR_GLOBAL_HOOK__ &&
			window.__PIXI_INSPECTOR_GLOBAL_HOOK__.register({
				PIXI: PIXI,
			})

		document.body.appendChild(this.application.view)

		const rect = new PIXI.Graphics()
		this.application.stage.addChild(rect)

		rect.beginFill(PIXI.utils.string2hex('#FCF3CF'))
		rect.drawRect(0, 0, 100, 100)
		rect.endFill()

		const interactionManager = this.application.renderer.plugins.interaction as PIXI.InteractionManager
	}
}
