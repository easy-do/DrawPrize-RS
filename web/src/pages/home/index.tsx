import React, { useEffect, useState } from 'react';
import useLocale from '@/utils/useLocale';
import locale from './locale';
import { Button, Card, Carousel, Divider, Grid, Link, List, Notification, } from '@arco-design/web-react';
import { drawLivePrizePool } from '@/api/livePrizePool';
const Row = Grid.Row;
const Col = Grid.Col;

export default function Welcome() {
  const t = useLocale(locale);

  const extra = <Link>More</Link>;

  const imageSrc = [
    'https://img0.baidu.com/it/u=3247766294,3714224757&fm=253&fmt=auto&app=138&f=JPEG?w=610&h=343',
    'https://img1.baidu.com/it/u=4113580494,1754073909&fm=253&fmt=auto&app=138&f=JPEG?w=650&h=360',
    'https://img1.baidu.com/it/u=2128492579,763816667&fm=253&fmt=auto&app=138&f=JPEG?w=780&h=360',
  ];

  return (
    <div
      style={{
        boxSizing: 'border-box',
        width: '100%',
        padding: 40,
        backgroundColor: 'var(--color-fill-2)',
      }}
    >
      <Row
        gutter={20}
        style={{ marginBottom: 20 }}
      >
        <Col>
          <Card
            title='公告'
            bordered={false}
            style={{
              width: '100%',
            }}
          >
            恭喜用户xxxxx获得一等奖
            恭喜用户xxxxx获得一等奖
            恭喜用户xxxxx获得一等奖
          </Card>
        </Col>

      </Row>

      <Row
        gutter={20}
        style={{ marginBottom: 20 }}
      >
        <Col span={16}>
          <Card
            title='抽奖'
            extra={<Link>奖池预览</Link>}
            bordered={false}
            style={{ width: '100%' }}
            actions={
              [
               <Button type='secondary' 
               onClick={
                ()=>drawLivePrizePool(1,1).then((res)=>{
                  const { success, message, data } = res.data;
                  if (success) {
                    Notification.success({ content: data, duration: 1000 });
                  } else {
                    Notification.error({ content: message, duration: 1000 });
                  }
                })
              } 
               shape='round'>单抽</Button>,
               <Button type='primary' 
               onClick={
                ()=>drawLivePrizePool(1,10).then((res)=>{
                  const { success, message, data } = res.data;
                  if (success) {
                    Notification.success({ content: data.join(','), duration: 3000 });
                  } else {
                    Notification.error({ content: message, duration: 1000 });
                  }
                })
              } 
               shape='round'>十连抽</Button>
              ]
            }
          >
            <Carousel
              autoPlay
              animation='card'
              showArrow='never'
              indicatorPosition='outer'
              style={{ width: '100%', height: 240 }}
              moveSpeed={1}
            >
              {imageSrc.map((src, index) => (
                <div
                  key={index}
                  style={{ width: '60%' }}
                >
                  <img
                    src={src}
                    style={{ width: '100%', height: '100%' }}
                  />
                </div>
              ))}
            </Carousel>
          </Card>
        </Col>
        <Col span={8}>
          <Card
            title='抽奖排行'
            bordered={false}
            style={{ width: '100%' }}
            size='small'
          >
            <List
              dataSource={[
                '张三  10次.',
                '张三  9.',
                '张三  8.',
                '张三  7.',
                '张三  6.',
                '张三  5.',
                '张三  4.',
                '张三  3.',
                '张三  2.',
                '张三  1.',
              ]}
              render={(item, index) => <List.Item key={index}>{item}</List.Item>}
            />
          </Card>
        </Col>
      </Row>
    </div>
  );
}

Welcome.displayName = 'HomePage';