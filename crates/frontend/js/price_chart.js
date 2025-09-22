export class MyChart {
    constructor() {
        this.chart = null;
        this.seriesBySymbol = new Map(); // symbol -> series
        this.dataBySymbol = new Map();   // symbol -> array of bars (LW format: {time, open, high, low, close})
    }

    draw(elementId) {
        const container = document.getElementById(elementId);
        if (!container) return;

        // destroy old chart if present
        if (this.chart) {
            try { this.chart.remove(); } catch (e) {}
            this.seriesBySymbol.clear();
            this.dataBySymbol.clear();
        }

        this.chart = LightweightCharts.createChart(container, {
            layout: {
                background: { color: '#111827' },
                textColor: '#d1d5db',
            },
            grid: {
                vertLines: { color: '#374151' },
                horzLines: { color: '#374151' },
            },
            timeScale: {
                timeVisible: true,
                secondsVisible: false,
            },
            crosshair: {
                mode: LightweightCharts.CrosshairMode.Normal,
            },
        });
    }

    updateData(symbol, timestamp_ms, open, high, low, close) {
        if (!this.chart) return;

        // create series for symbol if needed
        if (!this.seriesBySymbol.has(symbol)) {
            const series = this.chart.addCandlestickSeries({
                upColor: '#26a69a',
                borderUpColor: '#26a69a',
                wickUpColor: '#26a69a',
                downColor: '#ef5350',
                borderDownColor: '#ef5350',
                wickDownColor: '#ef5350',
            });
            this.seriesBySymbol.set(symbol, series);
            this.dataBySymbol.set(symbol, []);
        }

        // lightweight-charts expects UNIX seconds
        const time_secs = Math.floor(timestamp_ms / 1000);

        const bar = { time: time_secs, open, high, low, close };

        // update the chart series
        const series = this.seriesBySymbol.get(symbol);
        try {
            series.update(bar);
        } catch (e) {
            // fallback: setData if update fails for first points
            const arr = this.dataBySymbol.get(symbol) || [];
            arr.push(bar);
            series.setData(arr);
        }

        // persist in-memory history for this symbol
        const arr = this.dataBySymbol.get(symbol);
        arr.push(bar);
        if (arr.length > 500) arr.shift();
        this.dataBySymbol.set(symbol, arr);
    }

    // switchSymbol(symbol, optionalData)
    // If optionalData (array in ms timestamps) is provided it's used; otherwise uses cached data
    switchSymbol(symbol, data) {
        if (!this.chart) return;

        if (!this.seriesBySymbol.has(symbol)) {
            const s = this.chart.addCandlestickSeries({
                upColor: '#26a69a',
                downColor: '#ef5350',
            });
            this.seriesBySymbol.set(symbol, s);
            this.dataBySymbol.set(symbol, []);
        }

        const series = this.seriesBySymbol.get(symbol);

        if (Array.isArray(data) && data.length > 0) {
            const formatted = data.map(d => ({
                time: Math.floor(d.timestamp / 1000),
                open: d.open,
                high: d.high,
                low: d.low,
                close: d.close,
            }));
            series.setData(formatted);
            this.dataBySymbol.set(symbol, formatted);
            return;
        }

        const stored = this.dataBySymbol.get(symbol) || [];
        if (stored.length > 0) {
            series.setData(stored);
        } else {
            series.setData([]);
        }
    }

    destroy() {
        if (this.chart) {
            try { this.chart.remove(); } catch (e) {}
            this.chart = null;
            this.seriesBySymbol.clear();
            this.dataBySymbol.clear();
        }
    }
}
