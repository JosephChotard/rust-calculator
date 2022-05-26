import { globalStyle } from "@vanilla-extract/css"
import { theme } from "./theme"


globalStyle('*', {
  boxSizing: 'border-box',
  margin: 0,
  padding: 0,
})

globalStyle(':root', {
  backgroundColor: theme.contract.colors.background,
})