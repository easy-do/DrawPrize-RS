import React, { useEffect, useState } from 'react';
import { Descriptions, Drawer, Spin } from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { getPrizePoolInfo } from '@/api/prizePool';
import dayjs from 'dayjs';

function InfoPage(props: { id: number; visible; setVisible }) {
  const [loading, setLoading] = useState(false);

  const [infoData, setInfoData] = useState<any>();

  function fetchData() {
    setLoading(true);
    if (props.id !== undefined) {
      getPrizePoolInfo(props.id)
        .then((res) => {
          const { success, data } = res.data;
          if (success) {
            setInfoData(data);
          }
        })
        .finally(() => {
          setLoading(false);
        });
    }
  }

  useEffect(() => {
    fetchData();
  }, [props.id]);

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
              label: t['searchTable.columns.pool_name'],
              value: infoData ? infoData.pool_name : '',
            },
            {
              label: t['searchTable.columns.share_pool'],
              value: infoData
                ? infoData.share_pool
                  ? t['searchTable.columns.yes']
                  : t['searchTable.columns.no']
                : '',
            },
            {
              label: t['searchTable.columns.status'],
              value: infoData
                ? infoData.status
                  ? t['searchForm.enable']
                  : t['searchForm.disable']
                : '',
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
