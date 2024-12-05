'use strict';

var async = require('async');
var bitcore = require('bitcore-lib-btcz');
var Common = require('./common');
var LRU = require('lru-cache');

function ChartController(options) {
  var self = this;
  this.node = options.node;
  this.blocks = options.blocks;
  this.chartCache = LRU({
      max: options.chartCacheSize || ChartController.DEFAULT_CHART_CACHE_SIZE,
      maxAge: 1000 * 150
  });
  this.common = new Common({log: this.node.log});
}

var BLOCK_LIMIT = 200;

ChartController.DEFAULT_CHART_CACHE_SIZE = 5;
ChartController.CHARTS = {
  'block-size': {
    name: 'Block Size'
  },
  'block-interval': {
    name: 'Block Interval'
  },
  'difficulty': {
    name: 'Difficulty'
  },
  'mining-revenue': {
    name: 'Mining revenue'
  },
  'pool-stat': {
    name: 'Pool Stat'
  },
  'mined-block': {
    name: 'Mined Block'
  }
};

ChartController.prototype.list = function(req, res) {
  var data = {
    charts: ChartController.CHARTS
  };
  res.jsonp(data);
};

ChartController.prototype.chart = function(req, res, next) {
  var self = this;
  var chartType = req.params.chartType;

  if (!(chartType in ChartController.CHARTS)) {
    return self.common.handleErrors(null, res);
  }

    var dateStr;
    var todayStr = this.formatTimestamp(new Date());
    var isToday;

    if (req.query.blockDate) {
      dateStr = req.query.blockDate;
      var datePattern = /\d{4}-\d{2}-\d{2}/;
      if(!datePattern.test(dateStr)) {
        return self.common.handleErrors(new Error('Please use yyyy-mm-dd format'), res);
      }

      isToday = dateStr === todayStr;
    } else {
      dateStr = todayStr;
      isToday = true;
    }

    var gte = Math.round((new Date(dateStr)).getTime() / 1000);

    //pagination
    var lte = parseInt(req.query.startTimestamp) || gte + 86400;
    //var prev = this.formatTimestamp(new Date((gte - 86400) * 1000));
    //var next = lte ? this.formatTimestamp(new Date(lte * 1000)) : null;
    //var limit = parseInt(req.query.limit || BLOCK_LIMIT);
    //var more = false;
    //var moreTimestamp = lte;

  var cacheKey = chartType;
  var chartCached = self.chartCache.get(cacheKey);

 // if (chartCached && !req.query.blockDate) {
 //   req.chart = chartCached;
 //   console.log("Charts API 3 cached");
 //   next();
 // } else {
    self.node.services.bitcoind.getBlockHashesByTimestamp(lte, gte, function(err, hashes) {
      if (err) {
        return self.common.handleErrors(err, res);
      }
      async.mapSeries(
        hashes,
        function(hash, next) {
          var subReq = {
            params: {
              blockHash: hash
            }
          };
          self.blocks.block(subReq, res, function() {
            next(null, subReq.block);
          });
        },
        function(err, blocks) {
          if (err) {
            return self.common.handleErrors(err, res);
          }


          self.generateChart(chartType, blocks, function(err, chart) {
            if (err) {
              return self.common.handleErrors(err, res);
            }
            self.chartCache.set(cacheKey, chart);
            req.chart = chart;
            next();
          });
        }
      );
    });
  //}
};

ChartController.prototype.generateChart = function(chartType, blocks, callback) {
  try {
    if (chartType == 'mining-revenue') {
      this._miningRevenueChart(blocks, callback);
    } else if (chartType == 'pool-stat') {
      this._poolChart(blocks, callback);
    } else if (chartType == 'mined-block') {
      this._minedBlock(blocks, callback);
    } else {
      this._simpleChart(chartType, blocks, callback);
    }
  } catch (err){}
};

ChartController.prototype._poolChart = function(blocks, callback) {
  var self = this;
  async.mapSeries(
    blocks,

    function() {

      var poolNames =[];
      var poolBlks =[];
      var countSolo =0;

      blocks.forEach(element => {

        var poolName = element.poolInfo.poolName;
        if (poolName.charAt(0) == "t") {
         poolName="Others solo miners";
         countSolo += 1;
        };
        var dataVerif = poolNames.indexOf(poolName);
        if (dataVerif == -1) {
          poolNames.push(poolName);
          poolBlks.push(1);
        } else {
          poolBlks[dataVerif] += 1;
        }
      });

      poolNames[poolNames.indexOf("Others solo miners")] = "Others solo miners ("+countSolo+")";

      var chart = {
        name: ChartController.CHARTS['pool-stat'].name,
        data: {
          type:'pie',
          json: poolBlks,
          names: poolNames,
        }
      };

      callback(null, chart);
    }
  );
};

ChartController.prototype._minedBlock = function(blocks, callback) {
  var self = this;

  async.mapSeries(
    blocks,

    function() {

      var poolNames =[];
      var poolBlks =[];
      var countSolo =0;

      blocks.forEach(element => {
        var poolName = element.poolInfo.poolName;
        var dataVerif = poolNames.indexOf(poolName);
        if (dataVerif == -1) {
          poolNames.push(poolName);
          poolBlks.push(1);
        } else {
          poolBlks[dataVerif] += 1;
        }
      });

      var dataset = [];
      for (var i = 0; i < poolNames.length; i++) {
        dataset.push([poolNames[i], poolBlks[i]]);
      }

      var chart = {
        name: ChartController.CHARTS['mined-block'].name,
        data: {
          //x :'x',
          type:'bar',
          groups: [poolNames],
          columns: dataset,
        },
        axis: {
          rotated: true,
          x: {
              show: false,
              type: 'category',
              categories : ['Mined by'],
          },
          y: {
            label: 'Block count',
          },
        },
        legend: {
            show: false
        },
        tooltip: {
           grouped: false,
           show: true,
        },
      };

      callback(null, chart);
    }
  );
};

ChartController.prototype._miningRevenueChart = function(blocks, callback) {
  var self = this;
  async.mapSeries(
    blocks,
    function(block, next) {
      async.reduce(
        block.tx,
        block.reward * 1e8,
        function(memo, txid, next2) {
            self.node.getDetailedTransaction(txid, function(err, tx) {
                next(null, tx.outputSatoshis);
            });
        },
        function(err, revenueSatoshis) {
          next(err, revenueSatoshis);
        }
      );
    },
    function(err, revenuesSat) {
      var chart = {
        name: ChartController.CHARTS['mining-revenue'].name,
        data: {
          x: 'height',
          json: {
          },
          names: {
          }
        }
      };

      chart.data.json.height = blocks.map(function(block, index) {
        return block.height;
      });
      chart.data.names.height = 'Height';

      chart.data.json.revenue = revenuesSat.map(function(revenueSatoshis, index) {
        return (revenueSatoshis / 1e8).toFixed(8);
      });
      chart.data.names.revenue = 'Mining revenue';

      callback(null, chart);
    }
  );
};

ChartController.prototype._simpleChart = function(chartType, blocks, callback) {
  var chart = {
    name: ChartController.CHARTS[chartType].name,
    data: {
      x: 'height',
      json: {
      },
      names: {
      }
    }
  };

  chart.data.json.height = blocks.map(function(block, index) {
    return block.height;
  });
  chart.data.names.height = 'Height';

  if (chartType == 'block-size') {
    chart.data.json.size = blocks.map(function(block, index) {
      return block.size;
    });
    chart.data.names.size = 'Block size';
  } else if (chartType == 'block-interval') {
    chart.data.json.height = chart.data.json.height.slice(1);
    chart.data.json.blockinterval = blocks.slice(1).map(function(block, index) {
      return block.time - blocks[index].time;
    });
    chart.data.names.blockinterval = 'Block interval';
  } else if (chartType == 'difficulty') {
    chart.data.json.difficulty = blocks.map(function(block, index) {
      return block.difficulty;
    });
    chart.data.names.difficulty = 'Difficulty';
  }

  callback(null, chart);
};

ChartController.prototype.show = function(req, res) {
  if (req.chart) {
    res.jsonp(req.chart);
  }
};

//helper to convert timestamps to yyyy-mm-dd format
ChartController.prototype.formatTimestamp = function(date) {
  var yyyy = date.getUTCFullYear().toString();
  var mm = (date.getUTCMonth() + 1).toString(); // getMonth() is zero-based
  var dd = date.getUTCDate().toString();
  return yyyy + '-' + (mm[1] ? mm : '0' + mm[0]) + '-' + (dd[1] ? dd : '0' + dd[0]); //padding
};

module.exports = ChartController;
