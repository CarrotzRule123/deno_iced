import { ButtonElement } from "../lib/elements/button.ts";
import { Document } from "../mod.ts";

const document = new Document("Hello World")
const button = new ButtonElement()
document.body.appendChild(button)