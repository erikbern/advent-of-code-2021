from matplotlib import pyplot

n_lines = [len([line for line in open('src/bin/%02d.rs' % i)]) for i in range(1, 26)]
pyplot.style.use('ggplot')
pyplot.plot(n_lines, '.:')
pyplot.ylabel('Number of lines')
pyplot.xlabel('Day')
pyplot.savefig('plot.png', dpi=288)
