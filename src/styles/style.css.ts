import { globalStyle } from "@vanilla-extract/css"
import { theme } from "./theme"


globalStyle('*', {
  boxSizing: 'border-box',
  margin: 0,
  padding: 0,
})

globalStyle('#app', {
  backgroundColor: theme.contract.colors.background,
  height: '100vh',
  width: '100vw',
})