import { library } from "../main.ts";
import { Element, Elements } from "./element.ts";
import { ElementFromName } from "./util/util.ts";

export class Document {
    private elements: Element[] = []
    public body: DivElement

    constructor(title: string) {
        const encoder = new TextEncoder()
        const buffer = encoder.encode(title);

        library.symbols.ops_create_window(buffer, buffer.length)
    }

    public createElement(type: Elements) {
        this.elements.push(new Element(this.elements.length, ElementFromName(type)))
        library.symbols.ops_create_element(type)
    }
}