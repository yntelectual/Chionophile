package com.devonfw.application.queuemanagement.service.impl.rest;

import javax.inject.Inject;
import javax.inject.Named;

import org.springframework.data.domain.Page;

import com.devonfw.application.queuemanagement.logic.api.Queuemanagement;
import com.devonfw.application.queuemanagement.logic.api.to.QueueEto;
import com.devonfw.application.queuemanagement.logic.api.to.QueueSearchCriteriaTo;
import com.devonfw.application.queuemanagement.service.api.rest.QueuemanagementRestService;

/**
 * The service implementation for REST calls in order to execute the logic of component {@link Queuemanagement}.
 */
@Named("QueuemanagementRestService")
public class QueuemanagementRestServiceImpl implements QueuemanagementRestService {

  @Inject
  private Queuemanagement queuemanagement;

  @Override
  public QueueEto getQueue(long id) {

    return this.queuemanagement.findQueue(id);
  }

  @Override
  public QueueEto saveQueue(QueueEto queue) {

    return this.queuemanagement.saveQueue(queue);
  }

  @Override
  public long deleteQueue(long id) {

    this.queuemanagement.deleteQueue(id);
    return id;
  }

  @Override
  public Page<QueueEto> findQueues(QueueSearchCriteriaTo searchCriteriaTo) {

    return this.queuemanagement.findQueues(searchCriteriaTo);
  }
}