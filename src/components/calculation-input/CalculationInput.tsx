import styles from './CalculationInput.module.scss'

function CalculationInput() {

  return (
    <div className={styles.wrapper}>
      <input className={styles.input} type="text" placeholder="Enter a calculation" />
    </div>
  )
}

export default CalculationInput
