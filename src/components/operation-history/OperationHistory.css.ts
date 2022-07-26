import { style } from "@vanilla-extract/css"
import { sprinkles } from "../../styles"

export const historyContainer = style([
  sprinkles({
  }),
  {
    marginBottom: "4rem",
  }
])

export const historyItem = style([
  sprinkles({
  }),
  {
    display: "flex",
    flexDirection: "column",
    marginBottom: "1rem",
  }
])

export const result = style([
  sprinkles({
    color: {
      lightMode: 'yellow900',
      darkMode: 'yellow200',
    }
  }),
  {
    display: "inline",
    padding: 0,
    margin: 0,
    border: 0,
    background: "transparent",
    color: "inherit",
    font: "inherit",
    cursor: "pointer",
  }
])