import { style } from "@vanilla-extract/css"
import { sprinkles, theme } from "../../styles"
import { textColours } from "../../styles/theme"
import { makeTypographyRules } from "../typography/utils.css"

export const inputWrapper = style([
  sprinkles({
    color: textColours.strong,
    background: {
      lightMode: 'grey100',
      darkMode: 'grey800',
    },
  }),
  {
    position: 'relative',
    width: '100%',
    display: 'flex',
  }
])

export const input = style([
  makeTypographyRules(theme.text.standard).trimmed,
  sprinkles({
    color: textColours.strong,
  }),
  {
    padding: theme.spacing.xsmall,
    background: 'transparent',
    outline: 'none',
    border: 'none',
    flex: 1,
  }
])

export const response = style([
  sprinkles({
    background: {
      lightMode: 'grey800',
      darkMode: 'grey100',
    }
  }),
  {
    position: 'absolute',
    bottom: '100%',
    left: 0,
    padding: theme.spacing.xsmall,
    borderTopRightRadius: theme.border.radius.small,
  }
])