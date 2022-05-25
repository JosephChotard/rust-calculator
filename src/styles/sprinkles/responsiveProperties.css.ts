import { defineProperties } from "@vanilla-extract/sprinkles"
import { theme } from "../theme"

export const responsiveProperties = defineProperties({
  conditions: {
    mobile: {},
    tablet: { "@media": "screen and (min-width: 768px)" },
    desktop: { "@media": "screen and (min-width: 1024px)" }
  },
  defaultCondition: "mobile",
  properties: {
    fontSize: theme.fontSizes,
    lineHeight: theme.lineHeights
  },
  shorthands: {
    text: ["fontSize", "lineHeight"]
  }
})

