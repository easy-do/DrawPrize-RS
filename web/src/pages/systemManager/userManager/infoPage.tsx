import React, { useEffect, useState } from 'react';
import { Descriptions, Drawer, Spin } from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { getUserInfo } from '@/api/user';
import dayjs from 'dayjs';

function InfoPage(props: { id: number; visible; setVisible }) {
  const [loading, setLoading] = useState(false);

  const [infoData, setInfoData] = useState<any>();

  function fetchData() {
    setLoading(true);
    if (props.id !== undefined) {
      getUserInfo(props.id).then((res) => {
        const { success, data } = res.data;
        if (success) {
          setInfoData(data);
        }
      }).finally(()=>{
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
              label: t['searchTable.columns.user_name'],
              value: infoData ? infoData.user_name : '',
            },
            {
              label: t['searchTable.columns.nick_name'],
              value: infoData ? infoData.nick_name : '',
            },
            {
              label: t['searchTable.columns.status'],
              value: infoData
                ? infoData.status
                  ? t['searchForm.all.enable']
                  : t['searchForm.all.disable']
                : '',
            },
            {
              label: t['searchTable.columns.email'],
              value: infoData ? infoData.email : '',
            },
            {
              label: t['searchTable.columns.email_status'],
              value: infoData
                ? infoData.email_status
                  ? t['searchForm.all.enable']
                  : t['searchForm.all.notactive']
                : '',
            },
            {
              label: t['searchTable.columns.create_time'],
              value: infoData
                ? infoData.create_time
                  ? dayjs(infoData.create_time).format('YYYY-MM-DD HH:mm:ss')
                  : ''
                : '',
            },
            {
              label: t['searchTable.columns.last_login_time'],
              value: infoData
                ? infoData.last_login_time
                  ? dayjs(infoData.last_login_time).format(
                      'YYYY-MM-DD HH:mm:ss'
                    )
                  : ''
                : '',
            },
          ]}
        />
      </Spin>
    </Drawer>
  );
}

export default InfoPage;
