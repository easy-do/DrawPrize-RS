import {
  Form,
  FormInstance,
  Input,
  InputNumber,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { useContext, useEffect, useRef, useState } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { getCdkInfo, updateCdk } from '@/api/cdk';
import { getLivePrizePoolSelectList } from '@/api/livePrizePool';

function UpdatePage(props: { id: number; visible; setVisible; callback }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

    //所有开启的奖池
    const [poolSelectData, setPoolSelectData] = useState([]);

    useEffect(() => {
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
        }
      });
    }, []);

  //加载数据
  function fetchData() {
    if (props.id !== undefined && props.visible) {
      setLoading(true);
      getCdkInfo(props.id)
        .then((res) => {
          const { success, data } = res.data;
          if (success) {
            const ext_data_json = JSON.parse(data.ext_data);
            data.live_id = ext_data_json.live_id;
            data.draw_prize_times = ext_data_json.draw_prize_times;
            formRef.current.setFieldsValue(data);
          }
          setLoading(false);
        })
        .finally(() => {
          setLoading(false);
        });
    }
  }

  useEffect(() => {
    fetchData();
  }, [props.id, props.visible]);

  const t = useLocale(locale);

  //提交修改
  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.ext_data = JSON.stringify({
        live_id: values.live_id+'',
        draw_prize_times: Number(values.draw_prize_times),
      });
      updateCdk(values)
        .then((res) => {
          const { success, message } = res.data;
          if (success) {
            Notification.success({ content: message, duration: 1000 });
            props.setVisible(false);
            props.callback();
          }
        })
        .finally(() => {
          setLoading(false);
        });
    });
  };

  return (
    <Modal
      style={{ width: '35%' }}
      title={t['searchTable.update.title']}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          required
          initialValue={'1'}
          label={t['searchTable.columns.cdk_type']}
          field={'cdk_type'}
        >
          <Select
           options={[{
            label: 'default',
            value: '1',
           }]}
          />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.live_id']}
          field={'live_id'}
        >
          <Select
           options={poolSelectData}
          />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.draw_prize_times']}
          field={'draw_prize_times'}
        >
          <InputNumber type='number' min={1}  placeholder={t['searchForm.placeholder']} />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.desc']}
          field={'desc'}
        >
          <Input  placeholder={t['searchForm.placeholder']} />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default UpdatePage;
