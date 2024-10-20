import React, { useEffect, useState } from 'react';
import { Descriptions, Drawer, Spin } from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { getPrizePoolItemInfo } from '@/api/prizePoolItem';
import dayjs from 'dayjs';

function InfoPage(props: { info: any; visible; setVisible }) {
  const [loading, setLoading] = useState(false);

  const [infoData, setInfoData] = useState<any>();

  useEffect(() => {
    setInfoData(props.info);
  }, [JSON.stringify(props.info)]);

  const t = useLocale(locale);

  return (
    <Drawer
      width={'30%'}
      title={t['searchTable.columns.view']}
      visible={props.visible}
      onOk={() => {
        props.setVisible(false);
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      footer={null}
    >
      <Spin dot loading={loading}>
        <Descriptions
          colon=""
          //   title='Personal Information'
          column={1}
          labelStyle={{ width: 100 }}
          data={[
            {
              label: t['searchTable.columns.id'],
              value: infoData ? infoData.id : '',
            },
            {
              label: t['searchTable.columns.prize_ids'],
              value: infoData && infoData.prize_items ? JSON.parse(infoData.prize_items).map((item, index) => (
                    <>
                      <span key={index} style={{color:'red'}}>[{item.level_name}]</span>
                      <span key={index} style={{color:'blue'}}>{item.prize_name}</span>
                      <span key={index} style={{color:'red'}}>[cdk:{item.cdk}]</span>
                      <br></br>
                    </>
                  )) : ''
            },
            {
              label: t['searchTable.columns.create_time'],
              value:
                infoData && infoData.create_time
                  ? dayjs(infoData.create_time).format('YYYY-MM-DD HH:mm:ss')
                  : '',
            },
            {
              label: t['searchTable.columns.update_time'],
              value:
                infoData && infoData.update_time
                  ? dayjs(infoData.update_time).format('YYYY-MM-DD HH:mm:ss')
                  : '',
            },
            {
              label: t['searchTable.columns.pool_desc'],
              value: infoData ? infoData.pool_desc : '',
            },
          ]}
        />
      </Spin>
    </Drawer>
  );
}

export default InfoPage;
