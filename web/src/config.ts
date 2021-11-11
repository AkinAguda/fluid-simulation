export interface RangeConfigOptions {
  key: string;
  title: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onInput: (value: number) => void;
}

export class RangeConfig {
  private controller: HTMLDivElement;
  private valueInput: HTMLInputElement;
  private rangeInput: HTMLInputElement;
  private progress: HTMLDivElement;
  constructor(private options: RangeConfigOptions) {
    this.controller = document.createElement("div");
    const title = document.createElement("div");
    const controllers = document.createElement("div");
    const meter = document.createElement("div");
    this.valueInput = document.createElement("input");
    this.rangeInput = document.createElement("input");
    this.progress = document.createElement("div");
    const track = document.createElement("div");

    title.classList.add("title");
    controllers.classList.add("controllers");
    meter.classList.add("meter");
    this.valueInput.classList.add("value-box");
    this.rangeInput.classList.add("range");
    this.progress.classList.add("progress");
    track.classList.add("track");

    title.innerHTML = options.title;
    this.controller.appendChild(title);

    this.rangeInput.type = "range";
    meter.appendChild(this.rangeInput);
    this.rangeInput.max = options.max.toString();
    this.rangeInput.min = options.min.toString();
    this.rangeInput.step = options.step.toString();
    this.rangeInput.value = options.value.toString();
    this.rangeInput.oninput = this.onRangeInput;

    this.setProgressWidth(options.value);
    meter.appendChild(this.progress);
    meter.appendChild(track);

    controllers.appendChild(meter);

    this.valueInput.type = "number";
    controllers.appendChild(this.valueInput);

    this.valueInput.step = options.step.toString();
    this.rangeInput.max = options.max.toString();
    this.rangeInput.min = options.min.toString();
    this.valueInput.oninput = this.onValueInput;
    this.valueInput.value = options.value.toString();

    this.controller.appendChild(controllers);
  }

  getElement = () => {
    return this.controller;
  };

  private setProgressWidth = (value: number) => {
    this.progress.style.width = `${(
      (value / this.options.max) *
      100
    ).toString()}%`;
  };

  private onRangeInput = (e: any) => {
    const value = e.target.value;
    this.valueInput.value = value;
    this.setProgressWidth(value);
    this.options.onInput(parseFloat(value));
  };

  private onValueInput = (e: any) => {
    const value = e.target.value;
    if (value >= this.options.min && value <= this.options.max) {
      this.rangeInput.value = value;
      this.setProgressWidth(value);
      this.options.onInput(parseFloat(value));
    } else if (value < this.options.min) {
      this.valueInput.value = this.options.min.toString();
    } else if (value > this.options.max) {
      this.valueInput.value = this.options.max.toString();
    }
  };
}

export interface ButtonConfigOptions {
  title: string;
  onClick: (arg: any) => void;
}

export class ButtonConfig {
  private button: HTMLButtonElement;
  constructor(config: ButtonConfigOptions) {
    this.button = document.createElement("button");
    this.button.innerHTML = config.title;
    this.button.classList.add("config-button");
    this.button.onclick = config.onClick;
  }
  getElement = () => {
    return this.button;
  };
}

export class ConfigBox {
  configWrapper: HTMLDivElement;
  configTrigger: HTMLButtonElement;
  configDropdown: HTMLUListElement;
  constructor(private configs: Array<RangeConfig | ButtonConfig>) {
    this.configWrapper = document.getElementById(
      "config-wrapper"
    ) as HTMLDivElement;

    this.createConfigTrigger();
    this.createInputs();
  }

  createConfigTrigger = () => {
    this.configTrigger = document.createElement("button");
    this.configTrigger.setAttribute("id", "config-trigger");
    this.configTrigger.addEventListener("click", this.toggleConfig);
    this.configWrapper.appendChild(this.configTrigger);

    this.configDropdown = document.createElement("ul");
    this.configDropdown.setAttribute("id", "config-dropdown");
    this.configDropdown.classList.add("open");
    this.configWrapper.appendChild(this.configDropdown);
  };

  toggleConfig = () => {
    if (this.configDropdown.classList.contains("close")) {
      this.configDropdown.classList.replace("close", "open");
      this.configDropdown.style.display = "block";
    } else {
      this.configDropdown.classList.replace("open", "close");
      setTimeout(() => {
        this.configDropdown.style.display = "none";
      }, 300);
    }
  };

  createInputs = () => {
    this.configs.forEach((config) => {
      const li = document.createElement("li");
      li.appendChild(config.getElement());
      this.configDropdown.appendChild(li);
    });
  };
}
