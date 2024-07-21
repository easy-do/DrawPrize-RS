import React, { useEffect, useState } from 'react';
import useLocale from '@/utils/useLocale';
import locale from './locale';
import dayjs from 'dayjs';
import {
  Avatar,
  Button,
  Card,
  Grid,
  Link,
  List,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import {
  drawLivePrizePool,
  getDrawHistory,
  getLivePrizePoolSelectList,
  getPoolDrawCount,
  getPrizeItemList,
  topDraw,
} from '@/api/livePrizePool';
import ScrollableCard from './ScrollableCard';
import ScrollableCardList from './ScrollableCardList';
const Row = Grid.Row;
const Col = Grid.Col;

export default function Welcome() {
  const t = useLocale(locale);

  //预览奖池
  const [viewPoolItem, setviewPoolItem] = useState(false);

  //抽奖结果
  const [prizeResultVisible, setPrizeResultVisible] = useState(false);
  const [prizeResult, setPrizeResult] = useState([]);

  //抽奖排名
  const [topDrawData, setTopDrawData] = useState([]);
  //所有开启的奖池
  const [poolSelectData, setPoolSelectData] = useState([]);
  //当前奖池
  const [currentPoolId, setCurrentPoolId] = useState(1);
  //当前奖池物品
  const [currentPoolItemData, setCurrentPoolItemData] = useState([]);
  //当前奖池抽取和剩余奖品统计
  const [currentPoolDrawCountData, setCurrentPoolDrawCountData] = useState({action:0,remaining_quantity:0});
  //最新中奖信息
  const [drawHistoryData, setDrawHistoryData] = useState([]);

  function getDrawHistoryData() {
    getDrawHistory().then((res) => {
      const { success, data } = res.data;
      if (success) {
        setDrawHistoryData(data.map((item, index) => (
          dayjs(item.create_time).format('YYYY-MM-DD hh:mm:ss : ') + item.user_name+'  通过 '+item.action+' 抽获得 '+ JSON.parse(item.prize_items).map((item)=>item.prize_name).join("|"))
        ));
      }
    });
  }
  


  function poolSelectOnChange(value) {
    setCurrentPoolId(value);
    getCurrentPoolItemData(value);
    getTopDrawData();
    getCurrentPoolDrawCountData(value);
  }

  function getCurrentPoolItemData(currentPoolId) {
    getPrizeItemList(currentPoolId).then((res) => {
      const { success, data } = res.data;
      if (success) {
        setCurrentPoolItemData(data);
      }
    });
  }
  function getCurrentPoolDrawCountData(currentPoolId) {
    getPoolDrawCount(currentPoolId).then((res) => {
      const { success, data } = res.data;
      if (success) {
        setCurrentPoolDrawCountData(data);
      }
    });
  }

  function getTopDrawData() {
    topDraw().then((res) => {
      const { success, data } = res.data;
      if (success) {
        setTopDrawData(data);
      }
    });
  }

  useEffect(() => {
    getDrawHistoryData();
    getTopDrawData();
    getLivePrizePoolSelectList().then((res) => {
      const { success, data } = res.data;
      if (success && data.length > 0) {
        setPoolSelectData(
          data.map((item) => {
            return {
              label: item.pool_name,
              value: item.id,
            };
          })
        );
        if(data){
          setCurrentPoolId(data[0].id);
          getCurrentPoolItemData(data[0].id);
          getCurrentPoolDrawCountData(data[0].id);
        }
      }
    });
  }, []);

  return (
    <div
      style={{
        boxSizing: 'border-box',
        width: '100%',
        padding: 40,
        backgroundColor: 'var(--color-fill-2)',
      }}
    >
      <Row gutter={20} style={{ marginBottom: 20 }}>
        <Col>
          <ScrollableCard content={drawHistoryData}/>
        </Col>
      </Row>

      <Row gutter={20} style={{ marginBottom: 20 }}>
        <Col span={20}>
          <Card
            title={ <div>当前奖池累积抽取<span style={{color:'blue'}}>{currentPoolDrawCountData.action}</span> 次 剩余奖品 <span style={{color:'blue'}}>{currentPoolDrawCountData.remaining_quantity}</span></div>}
            extra={
              <>
                <Select placeholder="切换奖池" options={poolSelectData} onChange={poolSelectOnChange} />
              </>
            }
            bordered={false}
            style={{ width: '100%' }}
            actions={[
              <Button
                key={'one'}
                type="secondary"
                onClick={() =>
                  drawLivePrizePool(currentPoolId, 1).then((res) => {
                    const { success, message, data } = res.data;
                    if (success) {
                      setPrizeResult(data);
                      setPrizeResultVisible(true);
                      getTopDrawData();
                      getCurrentPoolItemData(currentPoolId);
                      getDrawHistoryData();
                      getCurrentPoolDrawCountData(currentPoolId);
                    } else {
                      Notification.error({ content: message, duration: 1000 });
                    }
                  })
                }
                shape="round"
              >
                单抽
              </Button>,
              <Button
                key={'two'}
                type="primary"
                onClick={() =>
                  drawLivePrizePool(currentPoolId, 10).then((res) => {
                    const { success, message, data } = res.data;
                    if (success) {
                      setPrizeResult(data);
                      setPrizeResultVisible(true);
                      getTopDrawData();
                      getCurrentPoolItemData(currentPoolId);
                      getDrawHistoryData();
                      getCurrentPoolDrawCountData(currentPoolId);
                    } else {
                      Notification.error({ content: message, duration: 1000 });
                    }
                  })
                }
                shape="round"
              >
                十连抽
              </Button>,
            ]}
          >
          <ScrollableCardList content={currentPoolItemData}/>
          </Card>
        </Col>
        <Col span={4}>
          <Card
            title="排行榜"
            bordered={false}
            style={{ width: '100%' }}
            size="small"
          >
            <List
              style={{minHeight:'550px', maxHeight:'550px'}} 
              dataSource={topDrawData}
              render={(item, index) => (
                <List.Item.Meta
                  key={index}
                  avatar={<Avatar shape="square">U</Avatar>}
                  title={<div>{'第' + (index + 1) + '名:'} <span style={{ color: 'blue' }}>{item.user_name}</span></div>}
                  description={
                    <div>
                      <span style={{ color: 'red' }}>{item.action}</span>
                      次
                    </div>
                  }
                />
              )}
            />
          </Card>
        </Col>
      </Row>

      <Modal
        title={'抽奖结果'}
        style={{ minWidth: '30%', minHeight: '30%' }}
        visible={prizeResultVisible}
        onCancel={() => setPrizeResultVisible(false)}
        footer={null}
      >
        <List
          style={{ minWidth: '30%', minHeight: '30%' }}
          dataSource={prizeResult}
          grid={{ gutter: 0, span: 8 }}
          render={(item, index) => (
              <List.Item.Meta
                key={index}
                avatar={<Avatar shape="square" size={90} >
                  <img src={item.icon} />
                </Avatar>}
                title={item.prize_name}
              />
            )}
        />
      </Modal>
    </div>
  );
}

Welcome.displayName = 'HomePage';
