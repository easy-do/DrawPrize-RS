import React, { useEffect, useState } from 'react';
import useLocale from '@/utils/useLocale';
import locale from './locale';
import dayjs from 'dayjs';
import {
  Avatar,
  Button,
  Card,
  Grid,
  Input,
  List,
  Modal,
  Notification,
  Select,
  Space,
} from '@arco-design/web-react';
import {
  drawLivePrizePool,
  getDrawHistory,
  getLivePrizePoolSelectList,
  getPoolDrawCount,
  getPrizeItemList,
  getUserDrawRemainingTimes,
  topDraw,
} from '@/api/livePrizePool';
import ScrollableCard from './ScrollableCard';
import ScrollableCardList from './ScrollableCardList';
import { requetUseCdk } from '@/api/cdk';
import checkLogin from '@/utils/checkLogin';
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
  const [userDrawRemainingTimes, setUserDrawRemainingTimes] = useState('-');

  //CDK兑换
  const [useCdktVisible, setUseCdktVisible] = useState(false);
  const [cdktValues, setCdktValues] = useState('');

  function getDrawHistoryData() {
    getDrawHistory().then((res) => {
      const { success, data } = res.data;
      if (success) {
        setDrawHistoryData(data.map((item, index) => (
          <div key={index}>
            {dayjs(item.create_time).format('YYYY-MM-DD hh:mm:ss : ') }
            用户 <span style={{color:'blue'}}>{item.user_name}</span>
            {'  通过 '}
              <span style={{color:'red'}}>{item.action}</span> 抽 获得:&nbsp;
            {
              JSON.parse(item.prize_items).map((item, index) => (
                <>
                  <span key={index} style={{color:'red'}}>[{item.level_name}]</span>
                  <span key={index} style={{color:'blue'}}>{item.prize_name}</span>&nbsp;
                </>
              ))
            }
          </div>
        )));
      }
    });
  }
  


  function poolSelectOnChange(value) {
    setCurrentPoolId(value);
    getCurrentPoolItemData(value);
    getTopDrawData();
    getCurrentPoolDrawCountData(value);
    getUserDrawRemainingTimesData(value);
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
  function getUserDrawRemainingTimesData(currentPoolId) {
    if(checkLogin()){
      getUserDrawRemainingTimes(currentPoolId).then((res) => {
        const { success, data } = res.data;
        if (success) {
          setUserDrawRemainingTimes(data);
        }
      });
    }
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
          getUserDrawRemainingTimesData(data[0].id);
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
            title={ <div>当前奖池累积抽取:<span style={{color:'blue'}}>{currentPoolDrawCountData.action}</span> 次 剩余奖品: <span style={{color:'blue'}}>{currentPoolDrawCountData.remaining_quantity}</span> 可抽次数: <span style={{color:'blue'}}>{userDrawRemainingTimes}</span></div>}
            extra={
              <>
              <Space>
                <Select placeholder="切换奖池" options={poolSelectData} onChange={poolSelectOnChange} />
              <Button type='primary' onClick={()=>setUseCdktVisible(true)}>CDK兑换</Button>
              </Space>
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
                      getUserDrawRemainingTimesData(currentPoolId);
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
                      getUserDrawRemainingTimesData(currentPoolId);
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
      <Modal
        title={'CDK兑换'}
        style={{ minWidth: '30%', minHeight: '30%' }}
        visible={useCdktVisible}
        onCancel={() => setUseCdktVisible(false)}
        onConfirm={() => {
          requetUseCdk(cdktValues.split('\n')).then((res) => {
            const { success, data } = res.data;
            if (success) {
              setUseCdktVisible(false);
              Notification.success({ content: '成功兑换'+data+'个', duration: 1000 });
            }
          });
        }}
      >
          <Input.TextArea style={{minHeight:'300px'}} defaultValue={cdktValues} onChange={(value)=>setCdktValues(value)}  placeholder='每行一个' allowClear />
      </Modal>
    </div>
  );
}

Welcome.displayName = 'HomePage';
