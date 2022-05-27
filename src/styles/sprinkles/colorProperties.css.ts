import { defineProperties } from "@vanilla-extract/sprinkles"
import { darkTheme, theme } from "../theme"


const availableColours = {
  ...theme.palette,
  ...theme.dynamicTheme.colors
}
export const colorProperties = defineProperties({
  conditions: {
    lightMode: {},
    darkMode: { selector: `.${darkTheme} &` },
  },
  defaultCondition: 'lightMode',
  properties: {
    background: availableColours,
    color: availableColours,
  },
})