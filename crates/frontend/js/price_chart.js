export class MyChart {
    chart;
    constructor() {
        this.config = {
            type: 'candlestick',
            data: {
                datasets: [{
                    label: 'Price',
                    data: [
                        { x: new Date('2025-09-01').getTime(), o: 150, h: 170, l: 140, c: 160 },
                        { x: new Date('2025-09-02').getTime(), o: 160, h: 175, l: 155, c: 165 },
                        { x: new Date('2025-09-03').getTime(), o: 165, h: 180, l: 160, c: 170 }
                    ]
                }]
            },
            options: {
                responsive: false,
                scales: {
                    x: {
                        type: 'time',
                        time: { unit: 'day' }
                    }
                }
            }
        };
    }

    draw(element_id) {
        this.chart = new Chart(
            document.getElementById(element_id),
            this.config
        )
    }
}