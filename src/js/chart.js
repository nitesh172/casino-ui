export function totalUsersChart(containerWidth, containerHeight, dateData) {
  let data = dateData;
  const margin = { top: 10, right: 10, bottom: 10, left: 20 };
  const width = containerWidth - margin.left - margin.right;
  const height = containerHeight - margin.top - margin.bottom;

  console.log(data);

  const xScale = d3
    .scaleBand()
    .domain(data.map((d) => d.key))
    .range([0, width])
    .padding(0.1);

  const yScale = d3.scaleLinear().domain([0, 10000]).range([height, 0]);

  const xAxis = d3.axisBottom(xScale).tickSize(0);
  const yAxis = d3
    .axisLeft(yScale)
    .tickSize(0)
    // .tickSizeOuter(0)
    .tickFormat((d) => `${d / 1000}k`);

  const line = d3
    .line()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y0(yScale(1000))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  // Remove existing SVG
  d3.selectAll('#total-users-chart svg').remove();
  console.log('Existing SVG removed');

  // Create a new SVG and add the chart
  const svg = d3
    .select('#total-users-chart')
    .append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);
  console.log('New SVG created');

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

  svg
    .append('path')
    .datum(data)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', line);

  const defaultMonth = data[0].key;

  const circles = svg
    .selectAll('circle')
    .data(data)
    .enter()
    .append('circle')
    .attr('cx', (d) => xScale(d.key) + xScale.bandwidth() / 2)
    .attr('cy', (d) => yScale(d.value))
    .attr('r', 8) // Adjust the radius as needed
    .attr('fill', 'red') // Change the color as needed
    .attr('class', 'data-point')
    .style('opacity', (d) => (d.key === defaultMonth ? 1 : 0)); // Set initial opacity

  let highlightedData = data.find((d) => d.key === defaultMonth);
  updateDataDiv(highlightedData);

  circles.on('click', function (event, d) {
    highlightedData = d;
    updateDataDiv(d);

    // Highlight the selected circle
    circles.style('opacity', (dataPoint) => (dataPoint.key === d.key ? 1 : 0));
  });

  svg.selectAll('.x-axis .tick rect').on('click', function (event, d) {
    const selectedMonth = d;

    // Update circle opacity based on the selected month
    circles.style('opacity', (dataPoint) =>
      dataPoint.key === selectedMonth ? 1 : 0
    );

    // Update highlighted data
    highlightedData = data.find((dataPoint) => dataPoint.key === selectedMonth);
    updateDataDiv(highlightedData);
  });

  function updateDataDiv(data) {
    // Update the content of the #users div
    d3.select('#users').text(`${data.value}`);
  }
}

export function createProfitLossChart(
  containerWidth,
  containerHeight,
  dateData
) {
  const profitData = dateData;

  const lossData = dateData;

  const margin = { top: 20, right: 30, bottom: 30, left: 40 };
  const width = containerWidth - margin.left - margin.right;
  const height = containerHeight - margin.top - margin.bottom;

  const circleOffset = 4; // Adjust the offset as needed

  const xScale = d3
    .scaleBand()
    .domain(profitData.map((d) => d.key))
    .range([0, width])
    .padding(0.1);

  const yScale = d3
    .scaleLinear()
    .domain([0, d3.max([...profitData, ...lossData], (d) => d.value)])
    .range([height, 0]);

  const yAxis = d3
    .axisLeft(yScale)
    .tickSize(0)
    .tickFormat((d) => `${d / 1000}k`);

  const profitLine = d3
    .line()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const lossLine = d3
    .line()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y0(yScale(0))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  // Remove existing SVG
  d3.selectAll('#profit-loss-chart-container svg').remove();
  console.log('Existing SVG removed');

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
    .call(d3.axisBottom(xScale).tickSize(0))
    .select('.domain')
    .remove();

  // Y-axis
  svg.append('g').call(yAxis).select('.domain').remove();

  // Gradient definition
  const profitGradient = svg
    .append('defs')
    .append('linearGradient')
    .attr('id', 'profitGradient')
    .attr('x1', '0%')
    .attr('y1', '0%')
    .attr('x2', '0%')
    .attr('y2', '100%');

  profitGradient
    .append('stop')
    .attr('offset', '10%')
    .attr('style', 'stop-color:#83BF94;stop-opacity:1');

  profitGradient
    .append('stop')
    .attr('offset', '100%')
    .attr('style', 'stop-color:#83BF94;stop-opacity:0');

  // Profit Area
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'url(#profitGradient)') // Green shade
    .attr('d', area)
    .style('pointer-events', 'none');

  // Profit Line
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'none')
    .attr('stroke', 'green')
    .attr('stroke-width', 2)
    .attr('d', profitLine)
    .style('pointer-events', 'none');

  // Gradient definition
  const lossGradient = svg
    .append('defs')
    .append('linearGradient')
    .attr('id', 'lossGradient')
    .attr('x1', '0%')
    .attr('y1', '0%')
    .attr('x2', '0%')
    .attr('y2', '100%');

  lossGradient
    .append('stop')
    .attr('offset', '10%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:1');

  lossGradient
    .append('stop')
    .attr('offset', '100%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:0');

  // Loss Area
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'url(#lossGradient)') // Red shade
    .attr('d', area)
    .style('pointer-events', 'none');

  // Loss Line
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', lossLine)
    .style('pointer-events', 'none');

  const defaultProfitMonth = profitData[0].key;
  const defaultLossMonth = lossData[0].key;

  let highlightedProfitData = profitData.find(
    (d) => d.key === defaultProfitMonth
  );

  updateProfitDataDiv(highlightedProfitData);
  let highlightedLossData = lossData.find((d) => d.key === defaultLossMonth);

  updateLossDataDiv(highlightedLossData);

  const profitCircles = svg
    .selectAll('.profit-circle')
    .data(profitData)
    .enter()
    .append('circle')
    .attr('cx', (d) => xScale(d.key) + xScale.bandwidth() / 2)
    .attr('cy', (d) => yScale(d.value) - circleOffset)
    .attr('r', 8) // Adjust the radius as needed
    .attr('fill', 'green') // Change the color as needed
    .attr('class', 'profit-circle')
    .style('opacity', (d) => (d.key === defaultProfitMonth ? 1 : 0))
    .style('pointer-events', 'all')
    .on('click', function (event, d) {
      highlightedProfitData = d;
      updateProfitDataDiv(d);

      // Highlight the selected circle
      profitCircles.style('opacity', (dataPoint) =>
        dataPoint.key === d.key ? 1 : 0
      );
    })
    .on('mouseover', function () {
      this.parentNode.appendChild(this);
    });

  const lossCircles = svg
    .selectAll('.loss-circle')
    .data(lossData)
    .enter()
    .append('circle')
    .attr('cx', (d) => xScale(d.key) + xScale.bandwidth() / 2)
    .attr('cy', (d) => yScale(d.value) + circleOffset)
    .attr('r', 8) // Adjust the radius as needed
    .attr('fill', 'red') // Change the color as needed
    .attr('class', 'loss-circle')
    .style('opacity', (d) => (d.key === defaultLossMonth ? 1 : 0))
    .style('pointer-events', 'all')
    .on('click', function (event, d) {
      highlightedLossData = d;
      updateLossDataDiv(d);

      // Highlight the selected circle
      lossCircles.style('opacity', (dataPoint) =>
        dataPoint.key === d.key ? 1 : 0
      );
    })
    .on('mouseover', function () {
      this.parentNode.appendChild(this);
    });

  function updateProfitDataDiv(data) {
    // Update the content of the #profit div
    d3.select('#totalProfit ').text(`${data.value}`);
  }

  function updateLossDataDiv(data) {
    // Update the content of the #loss div
    d3.select('#totalVolume').text(`${data.value}`);
  }
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
    .attr('width', 200)
    .attr('height', 200);

  // Set the center and radius of the circle
  const centerX = 100;
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
}

export function createTopupsCommissionChart(
  containerWidth,
  containerHeight,
  dateData
) {
  const profitData = dateData;

  const lossData = dateData;

  const margin = { top: 20, right: 30, bottom: 30, left: 40 };
  const width = containerWidth - margin.left - margin.right;
  const height = containerHeight - margin.top - margin.bottom;

  const circleOffset = 2; // Adjust the offset as needed

  const xScale = d3
    .scaleBand()
    .domain(profitData.map((d) => d.key))
    .range([0, width])
    .padding(0.1);

  const yScale = d3
    .scaleLinear()
    .domain([0, d3.max([...profitData, ...lossData], (d) => d.value)])
    .range([height, 0]);

  const yAxis = d3
    .axisLeft(yScale)
    .tickSize(0)
    .tickFormat((d) => `${d / 1000}k`);

  const profitLine = d3
    .line()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const lossLine = d3
    .line()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  const area = d3
    .area()
    .x((d) => xScale(d.key) + xScale.bandwidth() / 2)
    .y0(yScale(0))
    .y1((d) => yScale(d.value))
    .curve(d3.curveCardinal);

  // Remove existing SVG
  d3.selectAll('#topup-chart-container svg').remove();
  console.log('Existing SVG removed');

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
    .call(d3.axisBottom(xScale).tickSize(0))
    .select('.domain')
    .remove();

  // Y-axis
  svg.append('g').call(yAxis).select('.domain').remove();

  // Gradient definition
  const profitGradient = svg
    .append('defs')
    .append('linearGradient')
    .attr('id', 'profitGradient')
    .attr('x1', '0%')
    .attr('y1', '0%')
    .attr('x2', '0%')
    .attr('y2', '100%');

  profitGradient
    .append('stop')
    .attr('offset', '10%')
    .attr('style', 'stop-color:#83BF94;stop-opacity:1');

  profitGradient
    .append('stop')
    .attr('offset', '100%')
    .attr('style', 'stop-color:#83BF94;stop-opacity:0');

  // Profit Area
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'url(#profitGradient)') // Green shade
    .attr('d', area)
    .style('pointer-events', 'none');

  // Profit Line
  svg
    .append('path')
    .datum(profitData)
    .attr('fill', 'none')
    .attr('stroke', 'green')
    .attr('stroke-width', 2)
    .attr('d', profitLine)
    .style('pointer-events', 'none');

  // Gradient definition
  const lossGradient = svg
    .append('defs')
    .append('linearGradient')
    .attr('id', 'lossGradient')
    .attr('x1', '0%')
    .attr('y1', '0%')
    .attr('x2', '0%')
    .attr('y2', '100%');

  lossGradient
    .append('stop')
    .attr('offset', '10%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:1');

  lossGradient
    .append('stop')
    .attr('offset', '100%')
    .attr('style', 'stop-color:#C53A3A;stop-opacity:0');

  // Loss Area
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'url(#lossGradient)') // Red shade
    .attr('d', area)
    .style('pointer-events', 'none');

  // Loss Line
  svg
    .append('path')
    .datum(lossData)
    .attr('fill', 'none')
    .attr('stroke', 'red')
    .attr('stroke-width', 2)
    .attr('d', lossLine)
    .style('pointer-events', 'none');

  const defaultProfitMonth = profitData[0].key;
  const defaultLossMonth = lossData[0].key;

  let highlightedProfitData = profitData.find(
    (d) => d.key === defaultProfitMonth
  );

  updateProfitDataDiv(highlightedProfitData);

  let highlightedLossData = lossData.find((d) => d.key === defaultLossMonth);

  updateLossDataDiv(highlightedLossData);

  const profitCircles = svg
    .selectAll('.profit-circle')
    .data(profitData)
    .enter()
    .append('circle')
    .attr('cx', (d) => xScale(d.key) + xScale.bandwidth() / 2 - circleOffset)
    .attr('cy', (d) => yScale(d.value))
    .attr('r', 8) // Adjust the radius as needed
    .attr('fill', 'green') // Change the color as needed
    .attr('class', 'profit-circle')
    .style('opacity', (d) => (d.key === defaultProfitMonth ? 1 : 0))
    .style('pointer-events', 'all')
    .on('click', function (event, d) {
      highlightedProfitData = d;
      updateProfitDataDiv(d);

      // Highlight the selected circle
      profitCircles.style('opacity', (dataPoint) =>
        dataPoint.key === d.key ? 1 : 0
      );
    })
    .on('mouseover', function () {
      this.parentNode.appendChild(this);
    });

  const lossCircles = svg
    .selectAll('.loss-circle')
    .data(lossData)
    .enter()
    .append('circle')
    .attr('cx', (d) => xScale(d.key) + xScale.bandwidth() / 2 + circleOffset)
    .attr('cy', (d) => yScale(d.value))
    .attr('r', 8) // Adjust the radius as needed
    .attr('fill', 'red') // Change the color as needed
    .attr('class', 'loss-circle')
    .style('opacity', (d) => (d.key === defaultLossMonth ? 1 : 0))
    .style('pointer-events', 'all')
    .on('click', function (event, d) {
      highlightedLossData = d;
      updateLossDataDiv(d);

      // Highlight the selected circle
      lossCircles.style('opacity', (dataPoint) =>
        dataPoint.key === d.key ? 1 : 0
      );
    })
    .on('mouseover', function () {
      this.parentNode.appendChild(this);
    });

  // profitCircles.on('click', function (event, d) {
  //   highlightedProfitData = d;
  //   updateProfitDataDiv(d);

  //   this.parentNode.appendChild(this);

  //   // Highlight the selected circle
  //   profitCircles.style('opacity', (dataPoint) =>
  //     dataPoint.month === d.month ? 1 : 0
  //   );
  // });

  // // Add click event listener to loss circles
  // lossCircles.on('click', function (event, d) {
  //   highlightedLossData = d;
  //   updateLossDataDiv(d);

  //   this.parentNode.appendChild(this);

  //   // Highlight the selected circle
  //   lossCircles.style('opacity', (dataPoint) =>
  //     dataPoint.month === d.month ? 1 : 0
  //   );
  // });

  function updateProfitDataDiv(data) {
    // Update the content of the #profit div
    d3.select('#topups ').text(`${data.value}`);
  }

  function updateLossDataDiv(data) {
    // Update the content of the #loss div
    d3.select('#commission').text(`${data.value}`);
  }
}
