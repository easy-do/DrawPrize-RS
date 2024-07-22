import React, { useContext, useEffect, useRef, useState } from 'react';
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
import { addCdk } from '@/api/cdk';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { getLivePrizePoolSelectList } from '@/api/livePrizePool';

function AddPage(props: { visible; setVisible; callback: () => void }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

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

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.cdk_type = Number(values.cdk_type);
      values.ext_data = JSON.stringify({
        live_id: Number(values.live_id),
        draw_prize_times: Number(values.draw_prize_times),
      });
      addCdk(values).then((res) => {
        const { success, message } = res.data;
        if (success) {
          Notification.success({ content: message, duration: 1000 });
          props.setVisible(false);
          props.callback();
        }
      }).finally(()=>{
        setLoading(false);
      });
    });
  };

  return (
    <Modal
      title={t['searchTable.operations.add']}
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
          label={t['searchTable.columns.quantity']}
          field={'quantity'}
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

export default AddPage;
