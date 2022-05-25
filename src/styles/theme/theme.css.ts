import { createGlobalTheme, createTheme, createThemeContract } from "@vanilla-extract/css"

const rootTheme = createGlobalTheme("#app", {
  fonts: {
    body: "system-ui, sans-serif",
    heading: "system-ui, sans-serif",
  },
  space: {
    "0": "0",
    "1": "0.25rem",
    "2": "0.5rem",
    "3": "0.75rem",
    "4": "1rem",
  },
  fontSizes: {
    small: "16px",
    medium: "20px",
    large: "36px",
  },
  lineHeights: {
    small: "24px",
    medium: "28px",
    large: "40px",
  }
})

const tokens = {
  colors: {
    primary: null,
    secondary: null,
    background: null,
  }
}

export const contract = createThemeContract(tokens)

export const lightTheme = createTheme(contract, {
  colors: {
    primary: '#e6e6e6',
    secondary: '#00f0f0',
    background: '#fafafa',
  }
})

export const darkTheme = createTheme(contract, {
  colors: {
    primary: '#00f0f0',
    secondary: '#e6e6e6',
    background: "#202226",
  }
})

export const theme = {
  ...rootTheme,
  contract
}