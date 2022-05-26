import { defineProperties } from "@vanilla-extract/sprinkles"
import { darkTheme, theme } from "../theme"


export const colorProperties = defineProperties({
  conditions: {
    lightMode: {},
    darkMode: { selector: `.${darkTheme} &` },
  },
  defaultCondition: 'lightMode',
  properties: {
    background: theme.palette,
    color: theme.palette,
  },
})