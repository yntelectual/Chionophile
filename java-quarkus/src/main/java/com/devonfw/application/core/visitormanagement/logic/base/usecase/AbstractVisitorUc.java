package com.devonfw.application.visitormanagement.logic.base.usecase;

import javax.inject.Inject;

import com.devonfw.application.general.logic.base.AbstractUc;
import com.devonfw.application.visitormanagement.dataaccess.api.repo.VisitorRepository;

/**
 * Abstract use case for Visitors, which provides access to the commonly necessary data access objects.
 */
public abstract class AbstractVisitorUc extends AbstractUc {

  /** @see #getVisitorRepository() */
  @Inject
  private VisitorRepository visitorRepository;

  /**
   * @return the {@link VisitorRepository} instance.
   */
  public VisitorRepository getVisitorRepository() {

    return this.visitorRepository;
  }

}
