// Enhanced PriceChart with comprehensive technical indicators
// Add this script tag to your HTML head:
// <script src="https://unpkg.com/lightweight-charts/dist/lightweight-charts.standalone.development.js"></script>

export class PriceChart {
    constructor() {
        this.chart = null;
        this.seriesBySymbol = new Map(); // symbol -> main candlestick series
        this.dataBySymbol = new Map();   // symbol -> array of bars
        this.indicatorSeries = new Map(); // symbol -> Map of indicator name -> series
        this.indicatorData = new Map();   // symbol -> Map of indicator name -> data array
        this.activeIndicators = new Set(); // Set of active indicator names

        // Available indicators
        this.availableIndicators = {
            // Moving Averages
            'SMA_20': { name: 'SMA (20)', type: 'line', color: '#2196F3', params: { period: 20 } },
            'SMA_50': { name: 'SMA (50)', type: 'line', color: '#FF9800', params: { period: 50 } },
            'EMA_12': { name: 'EMA (12)', type: 'line', color: '#4CAF50', params: { period: 12 } },
            'EMA_26': { name: 'EMA (26)', type: 'line', color: '#F44336', params: { period: 26 } },

            // Bollinger Bands
            'BB': { name: 'Bollinger Bands', type: 'bands', color: '#9C27B0', params: { period: 20, stdDev: 2 } },

            // MACD
            'MACD': { name: 'MACD', type: 'oscillator', color: '#607D8B', params: { fast: 12, slow: 26, signal: 9 } },

            // RSI
            'RSI': { name: 'RSI (14)', type: 'oscillator', color: '#795548', params: { period: 14 } },

            // Stochastic
            'STOCH': { name: 'Stochastic', type: 'oscillator', color: '#FF5722', params: { k: 14, d: 3 } },

            // Volume indicators
            'VOLUME': { name: 'Volume', type: 'histogram', color: '#9E9E9E' },
            'OBV': { name: 'OBV', type: 'line', color: '#3F51B5' },

            // Momentum
            'MOMENTUM': { name: 'Momentum (10)', type: 'oscillator', color: '#009688', params: { period: 10 } },
            'ROC': { name: 'ROC (12)', type: 'oscillator', color: '#8BC34A', params: { period: 12 } },

            // Trend
            'ADX': { name: 'ADX (14)', type: 'oscillator', color: '#CDDC39', params: { period: 14 } },
            'CCI': { name: 'CCI (20)', type: 'oscillator', color: '#FFC107', params: { period: 20 } },

            // Support/Resistance
            'PIVOT': { name: 'Pivot Points', type: 'levels', color: '#E91E63' },
            'FIBONACCI': { name: 'Fibonacci', type: 'levels', color: '#673AB7' }
        };
    }

    draw(elementId) {
        console.log(`draw() called with elementId: ${elementId}`);
        const container = document.getElementById(elementId);
        if (!container) {
            console.error(`Container with id '${elementId}' not found!`);
            return;
        }

        if (typeof LightweightCharts === 'undefined') {
            console.error('LightweightCharts is not loaded. Add script tag to HTML.');
            return;
        }

        // Destroy old chart if present
        if (this.chart) {
            console.log("Destroying existing chart");
            try { this.chart.remove(); } catch (e) { console.warn("Error removing chart:", e); }
            this.seriesBySymbol.clear();
            this.dataBySymbol.clear();
            this.indicatorSeries.clear();
            this.indicatorData.clear();
        }

        console.log("Creating new chart...");
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
            rightPriceScale: {
                borderColor: '#485563',
            },
            leftPriceScale: {
                visible: false,
            },
        });

        console.log("Chart created successfully:", this.chart);
    }

    updateData(symbol, timestamp_ms, open, high, low, close, volume = 0) {
        console.log(`updateData called: symbol=${symbol}, timestamp=${timestamp_ms}, OHLCV=(${open}, ${high}, ${low}, ${close}, ${volume})`);

        if (!this.chart) {
            console.error("Chart is not initialized!");
            return;
        }

        // Create main series for symbol if needed
        if (!this.seriesBySymbol.has(symbol)) {
            console.log(`Creating new candlestick series for symbol: ${symbol}`);
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
            this.indicatorSeries.set(symbol, new Map());
            this.indicatorData.set(symbol, new Map());
        }

        const time_secs = Math.floor(timestamp_ms / 1000);
        const bar = { time: time_secs, open, high, low, close, volume };

        // Update main series
        const series = this.seriesBySymbol.get(symbol);
        try {
            series.update(bar);
        } catch (e) {
            console.warn("Series.update() failed, trying setData fallback:", e);
            const arr = this.dataBySymbol.get(symbol) || [];
            arr.push(bar);
            series.setData(arr);
        }

        // Update data array
        const arr = this.dataBySymbol.get(symbol) || [];
        arr.push(bar);
        if (arr.length > 1000) arr.shift(); // Keep last 1000 points
        this.dataBySymbol.set(symbol, arr);

        // Update all active indicators
        this.updateAllIndicators(symbol);
    }

    // Add or remove an indicator
    toggleIndicator(indicatorName, symbol) {
        if (this.activeIndicators.has(indicatorName)) {
            this.removeIndicator(indicatorName, symbol);
        } else {
            this.addIndicator(indicatorName, symbol);
        }
    }

    addIndicator(indicatorName, symbol) {
        if (!this.availableIndicators[indicatorName] || !this.chart) return;

        this.activeIndicators.add(indicatorName);

        const indicatorConfig = this.availableIndicators[indicatorName];
        const symbolIndicatorSeries = this.indicatorSeries.get(symbol) || new Map();
        const symbolIndicatorData = this.indicatorData.get(symbol) || new Map();

        // Create series based on indicator type
        let series;
        switch (indicatorConfig.type) {
            case 'line':
                series = this.chart.addLineSeries({
                    color: indicatorConfig.color,
                    lineWidth: 2,
                });
                break;
            case 'histogram':
                series = this.chart.addHistogramSeries({
                    color: indicatorConfig.color,
                    priceFormat: {
                        type: 'volume',
                    },
                    priceScaleId: 'volume',
                });
                break;
            case 'bands':
                // For Bollinger Bands, create three lines
                const upperSeries = this.chart.addLineSeries({
                    color: indicatorConfig.color,
                    lineWidth: 1,
                    lineStyle: LightweightCharts.LineStyle.Dashed,
                });
                const middleSeries = this.chart.addLineSeries({
                    color: indicatorConfig.color,
                    lineWidth: 2,
                });
                const lowerSeries = this.chart.addLineSeries({
                    color: indicatorConfig.color,
                    lineWidth: 1,
                    lineStyle: LightweightCharts.LineStyle.Dashed,
                });

                symbolIndicatorSeries.set(`${indicatorName}_upper`, upperSeries);
                symbolIndicatorSeries.set(`${indicatorName}_middle`, middleSeries);
                symbolIndicatorSeries.set(`${indicatorName}_lower`, lowerSeries);
                break;
            case 'oscillator':
                // Create a separate price scale for oscillators
                series = this.chart.addLineSeries({
                    color: indicatorConfig.color,
                    lineWidth: 2,
                    priceScaleId: indicatorName,
                });

                // Configure the price scale
                this.chart.priceScale(indicatorName).applyOptions({
                    scaleMargins: { top: 0.7, bottom: 0 },
                });
                break;
        }

        if (series && indicatorConfig.type !== 'bands') {
            symbolIndicatorSeries.set(indicatorName, series);
        }

        this.indicatorSeries.set(symbol, symbolIndicatorSeries);
        this.indicatorData.set(symbol, symbolIndicatorData);

        // Calculate and set initial data
        this.updateIndicator(indicatorName, symbol);
    }

    removeIndicator(indicatorName, symbol) {
        this.activeIndicators.delete(indicatorName);

        const symbolIndicatorSeries = this.indicatorSeries.get(symbol);
        if (symbolIndicatorSeries) {
            const indicatorConfig = this.availableIndicators[indicatorName];

            if (indicatorConfig.type === 'bands') {
                // Remove all three band series
                ['_upper', '_middle', '_lower'].forEach(suffix => {
                    const series = symbolIndicatorSeries.get(`${indicatorName}${suffix}`);
                    if (series) {
                        this.chart.removeSeries(series);
                        symbolIndicatorSeries.delete(`${indicatorName}${suffix}`);
                    }
                });
            } else {
                const series = symbolIndicatorSeries.get(indicatorName);
                if (series) {
                    this.chart.removeSeries(series);
                    symbolIndicatorSeries.delete(indicatorName);
                }
            }
        }

        const symbolIndicatorData = this.indicatorData.get(symbol);
        if (symbolIndicatorData) {
            symbolIndicatorData.delete(indicatorName);
        }
    }

    updateAllIndicators(symbol) {
        for (const indicatorName of this.activeIndicators) {
            this.updateIndicator(indicatorName, symbol);
        }
    }

    updateIndicator(indicatorName, symbol) {
        const data = this.dataBySymbol.get(symbol);
        if (!data || data.length === 0) return;

        const indicatorConfig = this.availableIndicators[indicatorName];
        const symbolIndicatorSeries = this.indicatorSeries.get(symbol);
        const symbolIndicatorData = this.indicatorData.get(symbol);

        if (!symbolIndicatorSeries || !indicatorConfig) return;

        let calculatedData;

        // Calculate indicator values
        switch (indicatorName) {
            case 'SMA_20':
            case 'SMA_50':
                calculatedData = this.calculateSMA(data, indicatorConfig.params.period);
                break;
            case 'EMA_12':
            case 'EMA_26':
                calculatedData = this.calculateEMA(data, indicatorConfig.params.period);
                break;
            case 'BB':
                calculatedData = this.calculateBollingerBands(data, indicatorConfig.params.period, indicatorConfig.params.stdDev);
                break;
            case 'MACD':
                calculatedData = this.calculateMACD(data, indicatorConfig.params.fast, indicatorConfig.params.slow, indicatorConfig.params.signal);
                break;
            case 'RSI':
                calculatedData = this.calculateRSI(data, indicatorConfig.params.period);
                break;
            case 'STOCH':
                calculatedData = this.calculateStochastic(data, indicatorConfig.params.k, indicatorConfig.params.d);
                break;
            case 'VOLUME':
                calculatedData = data.map(bar => ({ time: bar.time, value: bar.volume || 0 }));
                break;
            case 'OBV':
                calculatedData = this.calculateOBV(data);
                break;
            case 'MOMENTUM':
                calculatedData = this.calculateMomentum(data, indicatorConfig.params.period);
                break;
            case 'ROC':
                calculatedData = this.calculateROC(data, indicatorConfig.params.period);
                break;
            case 'ADX':
                calculatedData = this.calculateADX(data, indicatorConfig.params.period);
                break;
            case 'CCI':
                calculatedData = this.calculateCCI(data, indicatorConfig.params.period);
                break;
        }

        if (calculatedData) {
            symbolIndicatorData.set(indicatorName, calculatedData);

            // Update series based on type
            if (indicatorConfig.type === 'bands' && calculatedData.upper) {
                const upperSeries = symbolIndicatorSeries.get(`${indicatorName}_upper`);
                const middleSeries = symbolIndicatorSeries.get(`${indicatorName}_middle`);
                const lowerSeries = symbolIndicatorSeries.get(`${indicatorName}_lower`);

                if (upperSeries && middleSeries && lowerSeries) {
                    upperSeries.setData(calculatedData.upper);
                    middleSeries.setData(calculatedData.middle);
                    lowerSeries.setData(calculatedData.lower);
                }
            } else {
                const series = symbolIndicatorSeries.get(indicatorName);
                if (series && Array.isArray(calculatedData)) {
                    series.setData(calculatedData);
                }
            }
        }
    }

    // Technical Analysis Calculations

    calculateSMA(data, period) {
        const result = [];
        for (let i = period - 1; i < data.length; i++) {
            const sum = data.slice(i - period + 1, i + 1).reduce((acc, bar) => acc + bar.close, 0);
            result.push({ time: data[i].time, value: sum / period });
        }
        return result;
    }

    calculateEMA(data, period) {
        const result = [];
        const multiplier = 2 / (period + 1);
        let ema = data[0].close;

        result.push({ time: data[0].time, value: ema });

        for (let i = 1; i < data.length; i++) {
            ema = (data[i].close * multiplier) + (ema * (1 - multiplier));
            result.push({ time: data[i].time, value: ema });
        }
        return result;
    }

    calculateBollingerBands(data, period, stdDevMultiplier) {
        const sma = this.calculateSMA(data, period);
        const upper = [];
        const middle = [];
        const lower = [];

        for (let i = 0; i < sma.length; i++) {
            const dataIndex = i + period - 1;
            const prices = data.slice(dataIndex - period + 1, dataIndex + 1).map(bar => bar.close);

            const mean = sma[i].value;
            const variance = prices.reduce((acc, price) => acc + Math.pow(price - mean, 2), 0) / period;
            const stdDev = Math.sqrt(variance);

            const time = data[dataIndex].time;

            upper.push({ time, value: mean + (stdDev * stdDevMultiplier) });
            middle.push({ time, value: mean });
            lower.push({ time, value: mean - (stdDev * stdDevMultiplier) });
        }

        return { upper, middle, lower };
    }

    calculateMACD(data, fastPeriod, slowPeriod, signalPeriod) {
        const fastEMA = this.calculateEMA(data, fastPeriod);
        const slowEMA = this.calculateEMA(data, slowPeriod);

        const macdLine = [];
        const startIndex = slowPeriod - fastPeriod;

        for (let i = startIndex; i < fastEMA.length; i++) {
            const macdValue = fastEMA[i].value - slowEMA[i - startIndex].value;
            macdLine.push({ time: fastEMA[i].time, value: macdValue });
        }

        return macdLine;
    }

    calculateRSI(data, period) {
        const result = [];
        const gains = [];
        const losses = [];

        // Calculate initial gains and losses
        for (let i = 1; i <= period; i++) {
            const change = data[i].close - data[i - 1].close;
            gains.push(Math.max(change, 0));
            losses.push(Math.max(-change, 0));
        }

        let avgGain = gains.reduce((a, b) => a + b, 0) / period;
        let avgLoss = losses.reduce((a, b) => a + b, 0) / period;

        let rs = avgGain / avgLoss;
        let rsi = 100 - (100 / (1 + rs));
        result.push({ time: data[period].time, value: rsi });

        // Calculate remaining RSI values
        for (let i = period + 1; i < data.length; i++) {
            const change = data[i].close - data[i - 1].close;
            const gain = Math.max(change, 0);
            const loss = Math.max(-change, 0);

            avgGain = (avgGain * (period - 1) + gain) / period;
            avgLoss = (avgLoss * (period - 1) + loss) / period;

            rs = avgGain / avgLoss;
            rsi = 100 - (100 / (1 + rs));
            result.push({ time: data[i].time, value: rsi });
        }

        return result;
    }

    calculateStochastic(data, kPeriod, dPeriod) {
        const kValues = [];

        for (let i = kPeriod - 1; i < data.length; i++) {
            const slice = data.slice(i - kPeriod + 1, i + 1);
            const high = Math.max(...slice.map(bar => bar.high));
            const low = Math.min(...slice.map(bar => bar.low));
            const close = data[i].close;

            const k = ((close - low) / (high - low)) * 100;
            kValues.push({ time: data[i].time, value: k });
        }

        return kValues;
    }

    calculateOBV(data) {
        const result = [];
        let obv = 0;

        for (let i = 1; i < data.length; i++) {
            if (data[i].close > data[i - 1].close) {
                obv += data[i].volume || 0;
            } else if (data[i].close < data[i - 1].close) {
                obv -= data[i].volume || 0;
            }
            result.push({ time: data[i].time, value: obv });
        }

        return result;
    }

    calculateMomentum(data, period) {
        const result = [];
        for (let i = period; i < data.length; i++) {
            const momentum = data[i].close - data[i - period].close;
            result.push({ time: data[i].time, value: momentum });
        }
        return result;
    }

    calculateROC(data, period) {
        const result = [];
        for (let i = period; i < data.length; i++) {
            const roc = ((data[i].close - data[i - period].close) / data[i - period].close) * 100;
            result.push({ time: data[i].time, value: roc });
        }
        return result;
    }

    calculateADX(data, period) {
        // Simplified ADX calculation
        const result = [];
        const trueRanges = [];
        const plusDMs = [];
        const minusDMs = [];

        for (let i = 1; i < data.length; i++) {
            const high = data[i].high;
            const low = data[i].low;
            const prevHigh = data[i - 1].high;
            const prevLow = data[i - 1].low;
            const prevClose = data[i - 1].close;

            const tr = Math.max(high - low, Math.abs(high - prevClose), Math.abs(low - prevClose));
            const plusDM = high - prevHigh > prevLow - low ? Math.max(high - prevHigh, 0) : 0;
            const minusDM = prevLow - low > high - prevHigh ? Math.max(prevLow - low, 0) : 0;

            trueRanges.push(tr);
            plusDMs.push(plusDM);
            minusDMs.push(minusDM);
        }

        // Simplified moving average for demonstration
        for (let i = period - 1; i < trueRanges.length; i++) {
            const avgTR = trueRanges.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0) / period;
            const avgPlusDM = plusDMs.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0) / period;
            const avgMinusDM = minusDMs.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0) / period;

            const plusDI = (avgPlusDM / avgTR) * 100;
            const minusDI = (avgMinusDM / avgTR) * 100;
            const dx = Math.abs(plusDI - minusDI) / (plusDI + minusDI) * 100;

            result.push({ time: data[i + 1].time, value: dx });
        }

        return result;
    }

    calculateCCI(data, period) {
        const result = [];
        const constant = 0.015;

        for (let i = period - 1; i < data.length; i++) {
            const slice = data.slice(i - period + 1, i + 1);
            const typicalPrices = slice.map(bar => (bar.high + bar.low + bar.close) / 3);
            const sma = typicalPrices.reduce((a, b) => a + b, 0) / period;
            const meanDeviation = typicalPrices.reduce((acc, tp) => acc + Math.abs(tp - sma), 0) / period;

            const cci = (typicalPrices[typicalPrices.length - 1] - sma) / (constant * meanDeviation);
            result.push({ time: data[i].time, value: cci });
        }

        return result;
    }

    getAvailableIndicators() {
        return this.availableIndicators;
    }

    getActiveIndicators() {
        return Array.from(this.activeIndicators);
    }

    switchSymbol(symbol, data) {
        if (!this.chart) return;

        // Clear the chart
        this.chart.remove();

        // Recreate the chart
        const container = document.getElementById('liveChart');
        if (!container) return;

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

        // Create main series for the new symbol
        const series = this.chart.addCandlestickSeries({
            upColor: '#26a69a',
            borderUpColor: '#26a69a',
            wickUpColor: '#26a69a',
            downColor: '#ef5350',
            borderDownColor: '#ef5350',
            wickDownColor: '#ef5350',
        });

        this.seriesBySymbol.clear();
        this.seriesBySymbol.set(symbol, series);
        this.indicatorSeries.set(symbol, new Map());
        this.indicatorData.set(symbol, new Map());

        if (Array.isArray(data) && data.length > 0) {
            const formatted = data.map(d => ({
                time: Math.floor(d.timestamp / 1000),
                open: d.open,
                high: d.high,
                low: d.low,
                close: d.close,
                volume: d.volume || 0
            }));
            series.setData(formatted);
            this.dataBySymbol.set(symbol, formatted);
        } else {
            const stored = this.dataBySymbol.get(symbol) || [];
            if (stored.length > 0) {
                series.setData(stored);
            } else {
                series.setData([]);
                this.dataBySymbol.set(symbol, []);
            }
        }

        // Re-add active indicators
        const activeIndicatorsCopy = Array.from(this.activeIndicators);
        this.activeIndicators.clear();
        activeIndicatorsCopy.forEach(indicatorName => {
            this.addIndicator(indicatorName, symbol);
        });
    }

    destroy() {
        if (this.chart) {
            try { this.chart.remove(); } catch (e) {}
            this.chart = null;
            this.seriesBySymbol.clear();
            this.dataBySymbol.clear();
            this.indicatorSeries.clear();
            this.indicatorData.clear();
            this.activeIndicators.clear();
        }
    }


}