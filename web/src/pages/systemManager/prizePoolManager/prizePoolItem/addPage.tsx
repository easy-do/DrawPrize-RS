import React, { useContext, useEffect, useRef } from 'react';
import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { addPrizePoolItem } from '@/api/prizePoolItem';
import FormItem from '@arco-design/web-react/es/Form/form-item';

function AddPage(props: {
  visible;
  setVisible;
  prizePoolId;
  callback: () => void;
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);

  const [loading, setLoading] = React.useState(false);

  const handleSubmit = () => {
    formRef.current.validate().then((values) => {
      setLoading(true);
      values.status = values.status == 'true';
      values.level = Number(values.level);
      values.probability = Number(values.probability);
      values.quantity = Number(values.quantity);
      addPrizePoolItem(values)
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

  useEffect(() => {
    formRef.current?.setFieldsValue({ pool_id: props.prizePoolId });
  }, [props.visible]);

  return (
    <Modal
      title={t['searchTable.operations.add']}
      style={{ width: '35%' }}
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
          disabled
          required
          initialValue={props.prizePoolId}
          label={t['searchTable.columns.pool_id']}
          field={'pool_id'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.prize_name']}
          field={'prize_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem required label={t['searchTable.columns.icon']} field={'icon'}>
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.level']}
          field={'level'}
        >
          <Input
            type="number"
            placeholder={t['searchForm.placeholder']}
            allowClear
          />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.level_name']}
          field={'level_name'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.probability']}
          field={'probability'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.quantity']}
          field={'quantity'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.prize_desc']}
          field={'prize_desc'}
        >
          <Input placeholder={t['searchForm.placeholder']} allowClear />
        </FormItem>
        <FormItem
          required
          initialValue={'true'}
          label={t['searchTable.columns.status']}
          field={'status'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={[
              {
                label: t['searchForm.enable'],
                value: 'true',
              },
              {
                label: t['searchForm.disable'],
                value: 'false',
              },
            ]}
            allowClear
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default AddPage;
