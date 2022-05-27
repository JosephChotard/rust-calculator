import { style } from "@vanilla-extract/css"
import { sprinkles, theme } from "../../styles"
import { textColours } from "../../styles/theme"
import { makeTypographyRules } from "../typography/utils.css"

export const input = style([
  makeTypographyRules(theme.text.standard).trimmed,
  sprinkles({
    color: textColours.strong,
    background: {
      lightMode: 'grey100',
      darkMode: 'grey800',
    },
  }),
  {
    outline: 'none',
    border: 'none',
    padding: theme.spacing.xsmall
  }
])