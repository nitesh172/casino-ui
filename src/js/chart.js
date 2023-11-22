export function totalUsersChart(containerWidth, containerHeight) {
  const data = [
    { month: 'Jan', value: 3000 },
    { month: 'Feb', value: 3000 },
    { month: 'Mar', value: 3000 },
    { month: 'Apr', value: 3000 },
    { month: 'May', value: 3000 },
    { month: 'Jun', value: 3200 },
    { month: 'Jul', value: 3200 },
    { month: 'Aug', value: 8000 },
    { month: 'Sep', value: 8000 },
    { month: 'Oct', value: 8000 },
    { month: 'Nov', value: 8000 },
    { month: 'Dec', value: 8000 },
  ];

  const margin = { top: 10, right: 10, bottom: 10, left: 20 };
  const width = containerWidth - margin.left - margin.right;
  const height = containerHeight - margin.top - margin.bottom;

  const xScale = d3
    .scaleBand()
    .domain(data.map((d) => d.month))
    .range([0, width])
    .padding(0.1);

  const yScale = d3
    .scaleLinear()
    .domain([0, 10000]) // Y-axis range from 0 to 10k
    .range([height, 0]);

  const xAxis = d3.axisBottom(xScale).tickSize(0);
  const yAxis = d3
    .axisLeft(yScale)
    .tickSize(0)
    // .tickSizeOuter(0)
    .tickFormat((d) => `${d / 1000}k`);

  const line = d3
    .line()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y0(yScale(1000))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const svg = d3
    .select('#total-users-chart')
    .append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);

  // X-axis
  svg
    .append('g')
    .attr('transform', `translate(0,${height})`)
    .call(xAxis)
    .select('.domain')
    .remove();

  // Y-axis
  svg.append('g').call(yAxis).select('.domain').remove();

  // Gradient definition
  const gradient = svg
    .append('defs')
    .append('linearGradient')
    .attr('id', 'gradient')
    .attr('x1', '0%')
    .attr('y1', '0%')
    .attr('x2', '0%')
    .attr('y2', '100%');

  gradient
    .append('stop')
    .attr('offset', '10%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:1');

  gradient
    .append('stop')
    .attr('offset', '100%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:0');

  // Area
  svg
    .append('path')
    .datum(data)
    .attr('fill', 'url(#gradient)') // Use the gradient
    .attr('d', area);

  // Line
  let linePath = svg
    .append('path')
    .datum(data)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', line);
}

export function createProfitLossChart() {
  const profitData = [
    { month: 'Jan', value: 3000 },
    { month: 'Feb', value: 3000 },
    { month: 'Mar', value: 3000 },
    { month: 'Apr', value: 3000 },
    { month: 'May', value: 3000 },
    { month: 'Jun', value: 3200 },
    { month: 'Jul', value: 3200 },
    { month: 'Aug', value: 8000 },
    { month: 'Sep', value: 8000 },
    { month: 'Oct', value: 8000 },
    { month: 'Nov', value: 8000 },
    { month: 'Dec', value: 8000 },
  ];

  const lossData = [
    { month: 'Jan', value: 1000 },
    { month: 'Feb', value: 1500 },
    { month: 'Mar', value: 1500 },
    { month: 'Apr', value: 2000 },
    { month: 'May', value: 2000 },
    { month: 'Jun', value: 2000 },
    { month: 'Jul', value: 3200 },
    { month: 'Aug', value: 6000 },
    { month: 'Sep', value: 5000 },
    { month: 'Oct', value: 4000 },
    { month: 'Nov', value: 5000 },
    { month: 'Dec', value: 4000 },
  ];

  const margin = { top: 20, right: 30, bottom: 30, left: 40 };
  const width = 600 - margin.left - margin.right;
  const height = 360 - margin.top - margin.bottom;

  const xScale = d3
    .scaleBand()
    .domain(profitData.map((d) => d.month))
    .range([0, width])
    .padding(0.1);

  const yScale = d3
    .scaleLinear()
    .domain([0, d3.max([...profitData, ...lossData], (d) => d.value)])
    .range([height, 0]);

  const yAxis = d3.axisLeft(yScale).tickFormat((d) => `${d / 1000}k`);

  const profitLine = d3
    .line()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const lossLine = d3
    .line()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y0(yScale(0))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const svg = d3
    .select('#profit-loss-chart-container')
    .append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);

  // X-axis
  svg
    .append('g')
    .attr('transform', `translate(0,${height})`)
    .call(d3.axisBottom(xScale));

  // Y-axis
  svg.append('g').call(yAxis);

  // Profit Area
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'rgba(0, 255, 0, 0.2)') // Green shade
    .attr('d', area);

  // Profit Line
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'none')
    .attr('stroke', 'green')
    .attr('stroke-width', 2)
    .attr('d', profitLine);

  // Loss Area
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'rgba(255, 0, 0, 0.2)') // Red shade
    .attr('d', area);

  // Loss Line
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', lossLine);
}

export function createCircleChart() {
  // const margin = { top: 20, right: 20, bottom: 20, left: 20 };
  // const width = 400 - margin.left - margin.right;
  // const height = 400 - margin.top - margin.bottom;

  const data = [
    {
      label: 'Evolution',
      value: 5400,
    },
    {
      label: 'Eugi',
      value: 4520,
    },
    {
      label: 'Power',
      value: 2300,
    },
    {
      label: 'Participate',
      value: 4900,
    },
  ];

  const svg = d3
    .select('#circle-chart-container')
    .append('svg')
    .attr('width', 400)
    .attr('height', 200);

  // Set the center and radius of the circle
  const centerX = 200;
  const centerY = 100;
  const radius = 80;

  // Calculate total value
  const totalValue = d3.sum(data, (d) => d.value);

  const colorScale = d3.scaleOrdinal(d3.schemeCategory10);

  // Starting angle for the first segment
  let startAngle = 0;

  // Draw segments
  data.forEach((d) => {
    const endAngle = startAngle + (d.value / totalValue) * 2 * Math.PI;

    // Draw a filled arc for each segment
    svg
      .append('path')
      .attr(
        'd',
        d3
          .arc()
          .innerRadius(0)
          .outerRadius(radius)
          .startAngle(startAngle)
          .endAngle(endAngle)
      )
      // .attr('fill', colorScale(d.label))
      .attr('stroke', colorScale(d.label))
      .attr('stroke-width', 20)
      .attr('transform', `translate(${centerX},${centerY})`);

    // Update the starting angle for the next segment
    startAngle = endAngle;
  });

  svg
    .append('circle')
    .attr('cx', centerX)
    .attr('cy', centerY)
    .attr('r', 70)
    .attr('fill', 'white');

  // const labels = svg
  //   .selectAll('text')
  //   .data(pie(data))
  //   .enter()
  //   .append('text')
  //   .attr('transform', (d) => `translate(${arc.centroid(d)})`)
  //   .attr('text-anchor', 'middle')
  //   .text((d) => d.data.label);
}

export function createTopupsCommissionChart() {
  const profitData = [
    { month: 'Jan', value: 3000 },
    { month: 'Feb', value: 3000 },
    { month: 'Mar', value: 3000 },
    { month: 'Apr', value: 3000 },
    { month: 'May', value: 3000 },
    { month: 'Jun', value: 3200 },
    { month: 'Jul', value: 3200 },
    { month: 'Aug', value: 7000 },
    { month: 'Sep', value: 5000 },
    { month: 'Oct', value: 4000 },
    { month: 'Nov', value: 6000 },
    { month: 'Dec', value: 5000 },
  ];

  const lossData = [
    { month: 'Jan', value: 1000 },
    { month: 'Feb', value: 1500 },
    { month: 'Mar', value: 1500 },
    { month: 'Apr', value: 2000 },
    { month: 'May', value: 2000 },
    { month: 'Jun', value: 2000 },
    { month: 'Jul', value: 3200 },
    { month: 'Aug', value: 6000 },
    { month: 'Sep', value: 5000 },
    { month: 'Oct', value: 4000 },
    { month: 'Nov', value: 5000 },
    { month: 'Dec', value: 4000 },
  ];

  const margin = { top: 20, right: 30, bottom: 30, left: 40 };
  const width = 600 - margin.left - margin.right;
  const height = 400 - margin.top - margin.bottom;

  const xScale = d3
    .scaleBand()
    .domain(profitData.map((d) => d.month))
    .range([0, width])
    .padding(0.1);

  const yScale = d3
    .scaleLinear()
    .domain([0, d3.max([...profitData, ...lossData], (d) => d.value)]) // Y-axis range
    .range([height, 0]);

  const profitLine = d3
    .line()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const lossLine = d3
    .line()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.month) + xScale.bandwidth() / 2)
    .y0(yScale(0))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const svg = d3
    .select('#topup-chart-container')
    .append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);

  // X-axis
  svg
    .append('g')
    .attr('transform', `translate(0,${height})`)
    .call(d3.axisBottom(xScale));

  // Y-axis
  svg.append('g').call(d3.axisLeft(yScale));

  // Profit Area
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'rgba(0, 255, 0, 0.2)') // Green shade
    .attr('d', area);

  // Profit Line
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'none')
    .attr('stroke', 'green')
    .attr('stroke-width', 2)
    .attr('d', profitLine);

  // Loss Area
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'rgba(255, 0, 0, 0.2)') // Red shade
    .attr('d', area);

  // Loss Line
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', lossLine);
  console.log('here i am ');
}
