<script lang="js" setup>
import { Line } from 'vue-chartjs'

const props = defineProps(["data"]);

const getLabels = (data) => {
	let prev_label = null;
	return props.data.map(item => {
		const [y, m, d] = item.date.split('T')[0].split('-');

		if (prev_label != d) {
			prev_label = d;
			return d + "/" + m;
		}

		return '';
	});
}

const getEmoji = (rate) => {
	switch (rate) {
		case -3: return '☹️'; 
        case -2: return '🙁'; 
        case -1: return '😔'; 
        case 0: return '😐'; 
        case 1: return '🙂';
		case 2: return '😋';
		case 3: return '😍';
	}
}


const chartData = ref({
	labels: getLabels(props.data),
	datasets: [
		{
			label: 'Rate',
			data: props.data.map(item => item.rate),
			backgroundColor: '#9A97CB',
			borderColor: '#7E79BC',
			fill: false,
		},
	],
});

const chartOptions = ref({
	scales: {
		x: {
			ticks: {
				autoSkip: false,
				maxRotation: 90, 
				minRotation: 90,
				font: {
					family: 'JetBrains Mono',
					weight: '700', 
				},
			},
			grid: {
				display: false,
			},
			border: {
				display: false,
			},
		},
		y: {
			border: {
				display: false,
			},
			ticks: {
				callback: getEmoji,
				autoSkip: false,
				font: {
					family: 'JetBrains Mono',
					weight: '700',
				},
				stepSize: 1,
				min: -3,
				max: 3,
			},
			grid: {
				display: false,
			},
		},
	},
	layout: {
		padding: {
			bottom: 0,
		},
	},
	responsive: true,
	maintainAspectRatio: false,
	elements: {
		line: {
			tension: 0.5,
		},
		point: {
			radius: 4, 
		},
	},
	plugins: {
		legend: {
			display: false,
			labels: {
				font: {
					family: 'JetBrains Mono',
					weight: '500',
				},
			},
		},
	},

})

const display = useState("display", () => true);

const onResize = async () => {
    display.value = false;
	await nextTick();
	display.value = true;
}

onMounted(() => window.addEventListener("resize", onResize, true));


watch(props.data, async (newData) => {
	chartData.value.labels = getLabels(newData);
	chartData.value.datasets[0].data = newData.map(item => item.rate);

	chartData.value.labels.reverse();
	chartData.value.datasets[0].data.reverse();
	
	display.value = false;
	await nextTick();
	display.value = true;
}, { immediate: true });

</script>
<template>
	<div class="chart">
		<Line v-if="display" :data="chartData" :options="chartOptions" />
	</div>
</template>
<style lang="scss" scoped>
.chart {
	canvas {
		height: 200px;
		width: 100px;
		margin: 12px;
	}

	border-radius: 8px;
	background: #2a2a2d;
}
</style>
