import { Chart, Title, Tooltip, Legend, BarElement,PointElement, LineElement, CategoryScale, LinearScale } from 'chart.js'
export default defineNuxtPlugin(() => {
    Chart.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend ,LineElement, PointElement)
})